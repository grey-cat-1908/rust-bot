use serenity::builder::CreateEmbed;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{prelude::*, id::MessageId};
use serenity::prelude::*;
use serenity::utils::Colour;

#[command]
#[num_args(1)]
#[required_permissions("KICK_MEMBERS")]
#[bucket("moderation")]
#[description("Kick User")]
#[only_in("guilds")]
#[usage = "<@member>"]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member;

    member = args.single::<id::UserId>();

    let mut embed = CreateEmbed::default();

    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "User Was Not Found.").await.unwrap();
        }
        Ok(member_id) => {
            let user = member_id.to_user(&ctx).await.unwrap();

            let guild_member = ctx.cache.member(msg.guild_id.unwrap(), user.id).await.unwrap();
            let author_member = ctx.cache.member(msg.guild_id.unwrap(), msg.author.id).await.unwrap();

            let guild_member_top_role = guild_member.roles[0].to_role_cached(&ctx).await.unwrap().position;
            let author_member_top_role = author_member.roles[0].to_role_cached(&ctx).await.unwrap().position;

            if (guild_member_top_role >= author_member_top_role || user.id == msg.guild(&ctx).await.unwrap().owner_id) && msg.author.id != msg.guild(&ctx).await.unwrap().owner_id  {
                msg.channel_id
                    .say(&ctx, "You cannot kick this member.").await.unwrap();
            }
            else {
                embed
                    .title("Kick Operation - Done")
                    .color(Colour::ORANGE)
                    .description("User successfully kicked.");

                guild_member.kick(&ctx).await?;

                msg.channel_id.send_message(&ctx.http, |smv| {
                    smv.content("").embed(|em| {
                        em.0 = embed.0;
                        em
                    });
                    smv
                }).await.unwrap();
            }
        }
    }
    Ok(())
}

#[command]
#[num_args(1)]
#[required_permissions("BAN_MEMBERS")]
#[bucket("moderation")]
#[description("Ban User")]
#[only_in("guilds")]
#[usage = "<@member>"]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member;

    member = args.single::<id::UserId>();

    let mut embed = CreateEmbed::default();

    match member {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "User Was Not Found.").await.unwrap();
        }
        Ok(member_id) => {
            let user = member_id.to_user(&ctx).await.unwrap();

            let guild_member = ctx.cache.member(msg.guild_id.unwrap(), user.id).await.unwrap();
            let author_member = ctx.cache.member(msg.guild_id.unwrap(), msg.author.id).await.unwrap();

            let guild_member_top_role = guild_member.roles[0].to_role_cached(&ctx).await.unwrap().position;
            let author_member_top_role = author_member.roles[0].to_role_cached(&ctx).await.unwrap().position;

            if (guild_member_top_role >= author_member_top_role || user.id == msg.guild(&ctx).await.unwrap().owner_id) && msg.author.id != msg.guild(&ctx).await.unwrap().owner_id  {
                msg.channel_id
                    .say(&ctx, "You cannot ban this member.").await.unwrap();
            }
            else {
                embed
                    .title("Ban Operation - Done")
                    .color(Colour::ORANGE)
                    .description("User successfully banned.");

                guild_member.ban(&ctx, 0).await?;

                msg.channel_id.send_message(&ctx.http, |smv| {
                    smv.content("").embed(|em| {
                        em.0 = embed.0;
                        em
                    });
                    smv
                }).await.unwrap();
            }
        }
    }
    Ok(())
}

#[command]
#[num_args(1)]
#[required_permissions("MANAGE_MESSAGES")]
#[bucket("moderation")]
#[description("Delete number of messages in the chat")]
#[only_in("guilds")]
#[usage = "<number>"]
async fn purge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let number;

    number = args.single::<u64>();

    let mut embed1 = CreateEmbed::default();
    let mut embed2 = CreateEmbed::default();

    match number {
        Err(_) => {
            msg.channel_id
                .say(&ctx, "Please provide valid number argument (from 2 to 100) to run this command.").await.unwrap();
        }
        Ok(numbers_to) => {
            if numbers_to < 2 || numbers_to > 100 {
                msg.channel_id
                    .say(&ctx, "Please provide valid number argument (from 2 to 100) to run this command.").await.unwrap();
            }
            else {
                embed1
                    .title("Clear Operation - Started")
                    .color(Colour::ORANGE)
                    .description(format!("**Operation Started.** Finding and deleting {num} messages...", num=numbers_to));

                let mut find_msg = msg.channel_id.send_message(&ctx.http, |smv| {
                    smv.set_embed(embed1);
                    smv
                }).await.unwrap();

                let messages = msg.channel(ctx).await.unwrap().guild().unwrap().messages(ctx, |f| f.before(msg.id).limit(numbers_to)).await?;
                let message_ids = messages.iter().map(|m| m.id).collect::<Vec<MessageId>>();

                msg.channel_id.delete_messages(&ctx, &message_ids).await?;

                embed2
                    .title("Clear Operation - Done")
                    .color(Colour::ORANGE)
                    .description(format!("**Operation - Done.** Successfully deleted {num} messages!", num=message_ids.len()));

                find_msg.edit(&ctx.http, |smf| {
                    smf.set_embed(embed2);
                    smf
                }).await.unwrap();
            }
        }
    }
    Ok(())
}
