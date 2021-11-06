mod commands;

use std::{collections::HashSet, env, sync::Arc};
use sentry;

use commands::{meta::*, owner::*, moderation::*, fun::*};
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::{help_commands, macros::{group, help, hook}}, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::framework::standard::{Args, CommandGroup, CommandResult, DispatchError, HelpOptions};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use serenity::utils::Colour;
use tracing::{error, info};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[hook]
async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::Ratelimited(duration) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!("Try this again in {} seconds.", duration.as_secs()),
                )
                .await
                .unwrap();
        }
        DispatchError::NotEnoughArguments { min, given } => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!(
                        "This command required `{}` arguments.\nYou have only provided `{}`",
                        min, given
                    ),
                )
                .await
                .unwrap();
        }
        DispatchError::TooManyArguments { max, given } => {
            msg.channel_id
                .say(
                    &ctx.http,
                    &format!(
                        "This command only needs `{} `arguments.\nYou have only provided `{}`",
                        max, given
                    ),
                )
                .await
                .unwrap();
        }
        DispatchError::LackingPermissions(p) => {
            let mut base = String::from("You need the follwing permisison\n");
            let _p_vec = p
                .get_permission_names()
                .iter()
                .map(|f| -> String {
                    let app = format!("{}\n", f);
                    base.push_str(&app);
                    app
                })
                .collect::<Vec<String>>();
            msg.channel_id.say(&ctx.http, base).await.unwrap();
        }
        DispatchError::OnlyForOwners => {
            msg.channel_id
                .say(&ctx.http, "You don't have milk. bye, looser")
                .await
                .unwrap();
        }
        DispatchError::OnlyForGuilds => {
            msg.channel_id
                .say(&ctx.http, "Use this command only in guild channel, please.")
                .await
                .unwrap();
        }
        _ => {
            sentry::capture_message(
                &format!("Unhandled Error: {:?}", error),
                sentry::Level::Warning,
            );
            msg.channel_id
                .say(&ctx, "May I have some milk? (Unknown Error Occurred) Ask my developer please fixes please.")
                .await
                .unwrap();
        }
    }
}

#[group]
#[description("Meta Commands")]
#[commands(avatar, botinfo, userinfo, inviteinfo, calculator)]
struct Meta;

#[group]
#[description("Moderation Commands")]
#[commands(kick, ban, purge)]
struct Moderation;

#[group]
#[description("Fun Commands")]
#[commands(ship)]
struct Fun;

#[group]
#[description("Owner's Commands")]
#[commands(quit)]
struct Owners;

#[help]
#[individual_command_tip = "If you would like to get more information about a specific command or group, you can just pass it as a command argument; like so: `help botinfo`"]
#[strikethrough_commands_tip_in_guild = ""]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[lacking_permissions = "Hide"]
#[wrong_channel = "Hide"]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let mut ho = help_options.clone();
    ho.embed_error_colour = Colour::RED;
    ho.embed_success_colour = Colour::ORANGE;

    let _ = help_commands::with_embeds(ctx, msg, args, &ho, groups, owners).await;
    Ok(())

}



#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~")).help(&MY_HELP).on_dispatch_error(dispatch_error).group(&META_GROUP).group(&OWNERS_GROUP).group(&MODERATION_GROUP).group(&FUN_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}