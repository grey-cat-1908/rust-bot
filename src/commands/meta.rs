use serenity::builder::CreateEmbed;
use serenity::client::bridge::gateway::{ShardId};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{prelude::*};
use serenity::prelude::*;
use serenity::utils::Colour;
use crate::ShardManagerContainer;

use fasteval::error::Error;

#[command]
#[min_args(0)]
#[max_args(1)]
#[bucket("meta")]
#[description("Get user's avatar command.")]
#[only_in("guilds")]
#[usage = "[@member]"]
async fn avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member;
    let needed_var;

    if args.is_empty() {
        member = msg.author.id.to_user(&ctx).await;
    }
    else {
        needed_var = args.single::<id::UserId>();

        match needed_var {
            Err(_) => {
                member = msg.author.id.to_user(&ctx).await;
            }
            Ok(some_variable_2) => {
                member = some_variable_2.to_user(&ctx).await;
            }
        }
    }

    let mut embed = CreateEmbed::default();

    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "User Was Not Found.").await.unwrap();
        }
        Ok(member_id) => {
            embed
                .title(format!("User Avatar {member_name}", member_name=member_id.name))
                .color(Colour::ORANGE)
                .image(
                    &member_id
                        .avatar_url()
                        .unwrap_or(String::from(&member_id.default_avatar_url()))
                );

            msg.channel_id.send_message(&ctx.http, |smv| {
                smv.content("").embed(|em| {
                    em.0 = embed.0;
                    em
                });
                smv
            }).await.unwrap();
        }
    }


    Ok(())
}

#[command]
#[min_args(0)]
#[max_args(1)]
#[bucket("meta")]
#[description("Get information about user.")]
#[only_in("guilds")]
#[usage = "[@member]"]
async fn userinfo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member;

    if args.is_empty() {
        member = Ok(msg.author.id);
    }
    else {
        member = args.single::<id::UserId>();
    }

    let cached_guild = msg
        .guild_id
        .unwrap()
        .to_guild_cached(&ctx.cache)
        .await
        .unwrap();

    let mut embed = CreateEmbed::default();

    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "User Was Not Found.").await.unwrap();
        }
        Ok(member_id) => {
            let user = member.unwrap().to_user(&ctx).await.unwrap();
            let guild_member = ctx.cache.member(msg.guild_id.unwrap(), user.id).await.unwrap();

            embed
                .title(format!("Information about {member_name}", member_name=user.name))
                .color(Colour::ORANGE)
                .thumbnail(
                    &user
                        .avatar_url()
                        .unwrap_or(String::from(&user.default_avatar_url()))
                )
                .footer(|f| f.text(format!("ID: {}", user.id)));

            let status = match cached_guild.presences.get(&member_id) {
                Some(presence) => match presence.status {
                    OnlineStatus::Online => "<:online:905846524834115624> Online",
                    OnlineStatus::DoNotDisturb => "<:DoNotDisturb:905846366998265936> DND",
                    OnlineStatus::Idle => "<:idle:905846423709442139> IDLE",
                    OnlineStatus::Offline => "<:invisible_offline:905846466550067230> Offline",
                    OnlineStatus::Invisible => "<:invisible_offline:905846466550067230> Invisible",
                    _ => "Error",
                },
                None => "<:invisible_offline:905846466550067230> Offline",
            };

            let activities = match cached_guild.presences.get(&member_id) {
                Some(p) => p
                    .activities
                    .iter()
                    .map(|f| -> String {
                        let pre = match f.kind {
                            ActivityType::Competing => "Competing in",
                            ActivityType::Streaming => "Streaming",
                            ActivityType::Playing => "Playing",
                            ActivityType::Listening => "Listening to",
                            ActivityType::Custom => "",
                            _ => "Unknown",
                        };
                        format!("{} **{}**", pre, f.name)
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
                None => String::from("None"),
            };

            let nick = guild_member.nick;

            match nick {
                None => {
                    embed.field("Username", format!("{name}#{discriminator}", name=user.name, discriminator=user.discriminator), false);
                }
                Some(nick_name) => {
                    embed.field("Username", format!("{name}#{discriminator} ({nick})", name=user.name, discriminator=user.discriminator, nick=nick_name), false);
                }
            }

            embed.field("Status", status, true);
            if activities.len() > 5 {
                embed.field("Activities", activities, false);
            };

            embed.field("Created At", format!("<t:{created_date}:R>", created_date=user.created_at().timestamp()), true);
            embed.field("Joined At", format!("<t:{created_date}:R>", created_date=guild_member.joined_at.unwrap().timestamp()), true);

            msg.channel_id.send_message(&ctx.http, |smv| {
                smv.content("").embed(|em| {
                    em.0 = embed.0;
                    em
                });
                smv
            }).await.unwrap();
        }
    }


    Ok(())
}

#[command]
#[num_args(1)]
#[bucket("meta")]
#[description("Information about Server Invite")]
#[only_in("guilds")]
#[usage = "<code>"]
async fn inviteinfo(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let string_code: String = args.single().unwrap();

    let invite = ctx.http.get_invite(&string_code, true).await;

    let mut embed = CreateEmbed::default();

    match invite {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Invite Was Not Found.").await.unwrap();
        }
        Ok(invite_data) => {
            let unwarped_guild = invite_data.guild.as_ref().unwrap();

            embed
                .title(format!("{name} (ID: {id})", name=unwarped_guild.name, id=unwarped_guild.id))
                .url(invite_data.url())
                .color(Colour::ORANGE)
                .description(format!("**Invite Channel:** `{channel_name}` (ID: {channel_id})", channel_name=invite_data.channel.name, channel_id=invite_data.channel.id));

            msg.channel_id.send_message(&ctx.http, |smv| {
                smv.content("").embed(|em| {
                    em.0 = embed.0;
                    em
                });
                smv
            }).await?;
        }
    }

    Ok(())
}

#[command]
#[min_args(0)]
#[max_args(1)]
#[bucket("meta")]
#[description("Some Bot Information")]
#[only_in("guilds")]
#[usage = ""]
async fn botinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let latency = {
        let data_read = ctx.data.read().await;
        let shard_manager = data_read.get::<ShardManagerContainer>().unwrap();

        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;

        let runner = runners.get(&ShardId(ctx.shard_id)).unwrap();

        if let Some(duration) = runner.latency {
            format!("{:.2} ms", duration.as_millis())
        } else {
            "? ms".to_string()
        }
    };

    let mut embed = CreateEmbed::default();

    embed
        .title("Bot Information")
        .description("Simple Bot on Rust")
        .color(Colour::ORANGE)
        .fields(vec![
            ("Shard", format!("#{shard_id}", shard_id=ctx.shard_id), true),
            ("Latency", latency, true),
            ("Servers", format!("{servers}", servers=ctx.cache.guild_count().await), true),
            // ("Юзеров", format!("{servers}", servers=ctx.cache.user_count().await), true),
        ]);

    msg.channel_id.send_message(&ctx.http, |smv| {
        smv.content("").embed(|em| {
            em.0 = embed.0;
            em
        });
        smv
    }).await?;

    Ok(())
}

#[command]
#[min_args(1)]
#[bucket("meta")]
#[aliases(calc)]
#[description("Calculator")]
#[only_in("guilds")]
#[usage = "<Expression>"]
async fn calculator(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut operation = args.message().to_string();

    operation = operation.replace("**", "^");
    operation = operation.replace("pi()", "pi");
    operation = operation.replace("pi", "pi()");
    operation = operation.replace("π", "pi()");
    operation = operation.replace("euler", "e()");

    let mut operation_without_markdown = operation.replace(r"\\", r"\\\\");

    for i in &["*", "`", "_", "~", "|"] {
        operation_without_markdown = operation_without_markdown.replace(i, &format!(r"\{}", i));
    }

    let mut cb = |name: &str, args: Vec<f64>| -> Option<f64> {
        match name {
            "sqrt" => {
                let a = args.get(0);
                if let Some(x) = a {
                    let l = x.log10();
                    Some(10.0_f64.powf(l / 2.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    let val = fasteval::ez_eval(&operation, &mut cb);

    match val {
        Err(why) => {
            let text = match &why {
                Error::SlabOverflow => "Too many Expressions/Values/Instructions were stored.".to_string(),
                Error::EOF => "Reached an unexpected End Of Input during parsing.\nMake sure your operators are complete.".to_string(),
                Error::EofWhileParsing(x) => format!("Reached an unexpected End Of Input during parsing:\n{}", x),
                Error::Utf8ErrorWhileParsing(_) => "The operator could not be decoded with UTF-8".to_string(),
                Error::TooLong => "The expression is too long.".to_string(),
                Error::TooDeep => "The expression is too recursive.".to_string(),
                Error::UnparsedTokensRemaining(x) => format!("An expression was parsed, but there is still input data remaining.\nUnparsed data: {}", x),
                Error::InvalidValue => "A value was expected, but invalid input data was found.".to_string(),
                Error::ParseF64(x) => format!("Could not parse a 64 bit floating point number:\n{}", x),
                Error::Expected(x) => format!("The expected input data was not found:\n{}", x),
                Error::WrongArgs(x) => format!("A function was called with the wrong arguments:\n{}", x),
                Error::Undefined(x) => format!("The expression tried to use an undefined variable or function, or it didn't provide any required arguments.:\n{}", x),
                Error::Unreachable => "This error should never happen, if it did, contact my developer immediately!".to_string(),
                _ => format!("An unhandled error occurred:\n{:#?}", &why),
            };

            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Calculator - ERROR");
                        e.color(Colour::RED);
                        e.description(text)
                    })
                })
                .await?;
        }
        Ok(res) => {
            msg.channel_id
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Calculator - Result");
                        e.color(Colour::ORANGE);
                        e.description(res)
                    })
                })
                .await?;
        }
    }
    Ok(())
}
