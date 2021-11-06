use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{prelude::*};
use serenity::prelude::*;
use serenity::utils::Colour;

async fn get_number_from_string(string: String) -> i32 {
    let bytes = string.as_bytes();
    let mut ret: i32 = 50;
    for b in 1..bytes.len() {
        ret = ret + bytes[b - 1] as i32;
    }
    ret
}

async fn divide_until(mut n: i32, until: i32, by: i32) -> i32 {
    while n > until {
        n = n / by
    }
    n
}

#[command]
#[num_args(1)]
#[bucket("fun")]
#[description("Whats a ship")]
#[only_in("guilds")]
#[usage = "<@member>"]
async fn ship(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let second_member;

    second_member = args.single::<id::UserId>();

    let mut embed = CreateEmbed::default();

    match second_member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "User Was Not Found.").await.unwrap();
        }
        Ok(second_member_id) => {
            let user = second_member_id.to_user(&ctx).await.unwrap();

            let first = format!("{}", msg.author.name);
            let second = format!("{}", user.name);

            let to_compare: String = if &first > &second {
                first.to_owned() + &second
            } else {
                second.to_owned() + &first
            };

            let percent = divide_until(get_number_from_string(to_compare).await, 100, 2).await;

            let exclamatory_message = match percent {
                0..=39 => "Not soo good.",
                40..=59 => "Maybe it's great, but I don't think so.",
                60..=68 => "nice!",
                69..=76 => "Really Cool!",
                77..=97 => "Cool! It's amazing!",
                98..=100 => "Wow! I don't believe it!",
                _ => "who am I?",
            };

            embed
                .title("It's time to ship!")
                .color(Colour::ORANGE)
                .description(format!("These users (<@{}> and <@{}>) seem to be {}% compatible! {}", msg.author.id, user.id, percent, exclamatory_message));

            msg.channel_id.send_message(&ctx.http, |smv| {
                smv.embed(|em| {
                    em.0 = embed.0;
                    em
                });
                smv
            }).await.unwrap();
        }
    }
    Ok(())
}