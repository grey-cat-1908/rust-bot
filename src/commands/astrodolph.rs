use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::{prelude::*};
use serenity::prelude::*;
use serenity::utils::Colour;
use crate::DatabasePool;

#[command]
#[num_args(0)]
#[bucket("astrodolph")]
#[description("Get your own AstroDolph")]
#[only_in("guilds")]
async fn unew(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data_read = ctx.data.read().await;
        data_read.get::<DatabasePool>().unwrap().clone()
    };

    let query = sqlx::query!(
        "SELECT * FROM users WHERE id = $1", msg.author.id.to_string()
    )
        .fetch_optional(&pool)
        .await?;

    let mut embed = CreateEmbed::default();

    if let Some(_row) = query {
        embed
            .title("NO STOP PLEASE")
            .color(Colour::ORANGE)
            .description("Your astrodolph would be offended if you got a new one! ")
            .footer(|f| {
                f.text(format!("Bad man name is {}", msg.author.name));

                f
            });

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    } else {
        sqlx::query!(
                "INSERT INTO users VALUES ($1, $2, $3, $4, $5, $6)",
                msg.author.id.to_string(), 0, 1, 1, 0, 1
            )
            .execute(&pool)
            .await?;

        embed
            .title("New astrodolph found!")
            .color(Colour::ORANGE)
            .description("You found your astrodolph! Congratulations!")
            .thumbnail("https://media.discordapp.net/attachments/820210225775902770/913435832554233886/Background_1.png?width=986&height=898")
            .footer(|f| {
                f.text(format!("Good man name is {}", msg.author.name));

                f
            });

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    }
    Ok(())
}

#[command]
#[max_args(1)]
#[bucket("astrodolph")]
#[description("Check user's astrodolph stats")]
#[only_in("guilds")]
#[usage = "[@member]"]
async fn ustats(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
            let user_id = member_id.id.to_string();

            let pool = {
                let data_read = ctx.data.read().await;
                data_read.get::<DatabasePool>().unwrap().clone()
            };

            let query = sqlx::query!(
        "SELECT * FROM users WHERE id = $1", user_id
    )
                .fetch_optional(&pool)
                .await?;

            if let Some(row) = query {

                embed
                    .title("User's astrodolph information")
                    .color(Colour::ORANGE)
                    .thumbnail("https://media.discordapp.net/attachments/820210225775902770/913435832554233886/Background_1.png?width=986&height=898")
                    .description(format!("Available points: {}\nHealth: {}\nStrong: {}\n Wins: {}", row.balance.unwrap_or_default(),
                                                                            (((100 as f32) * 1.25 * (row.health as f32)) as i32),
                                                                            (((100 as f32) * 1.25 * (row.strong as f32)) as i32),
                                                                            row.wins.unwrap_or_default()))
                    .footer(|f| {
                        f.text(format!("AstroDolph owner's name is {}", msg.author.name));

                        f
                    });

                msg.channel_id.send_message(&ctx.http, |smv| {
                    smv.content("").embed(|em| {
                        em.0 = embed.0;
                        em
                    });
                    smv
                }).await.unwrap();
            } else {
                msg.channel_id
                    .say(&ctx, "Astrodolph was not found.").await.unwrap();
            }

        }
    }
    Ok(())
}

#[command]
#[num_args(0)]
#[bucket("astrodolph")]
#[description("Upgrade your AstroDolph's health")]
#[only_in("guilds")]
async fn upgrhp(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data_read = ctx.data.read().await;
        data_read.get::<DatabasePool>().unwrap().clone()
    };

    let query = sqlx::query!(
        "SELECT * FROM users WHERE id = $1", msg.author.id.to_string()
    )
        .fetch_optional(&pool)
        .await?;

    let mut embed = CreateEmbed::default();

    if let Some(row) = query {
        if row.balance.unwrap_or_default() <= 0 {
            embed
                .title("You don't have enough points")
                .color(Colour::ORANGE)
                .description("You need to win someone to get points")
                .footer(|f| {
                    f.text(format!("Request author name is {}", msg.author.name));

                    f
                });
        }
        else {
            sqlx::query!("UPDATE users SET balance = (balance - 1), health = (health + 1) WHERE id = $1;",
                msg.author.id.to_string())
                .execute(&pool)
                .await?;

            embed
                .title("Yeah!!!!")
                .color(Colour::ORANGE)
                .description("You have successfully upgraded your astrodolph. (health tree)")
                .footer(|f| {
                    f.text(format!("Request author name is {}", msg.author.name));

                    f
                });
        }

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    } else {
        embed
            .title("who")
            .color(Colour::ORANGE)
            .description("You don't have your own astrodolph...")
            .footer(|f| {
                f.text(format!("Request author name is {}", msg.author.name));

                f
            });

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    }
    Ok(())
}

#[command]
#[num_args(0)]
#[bucket("astrodolph")]
#[description("Upgrade your AstroDolph's strong")]
#[only_in("guilds")]
async fn upgrstrong(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data_read = ctx.data.read().await;
        data_read.get::<DatabasePool>().unwrap().clone()
    };

    let query = sqlx::query!(
        "SELECT * FROM users WHERE id = $1", msg.author.id.to_string()
    )
        .fetch_optional(&pool)
        .await?;

    let mut embed = CreateEmbed::default();

    if let Some(row) = query {
        if row.balance.unwrap_or_default() <= 0 {
            embed
                .title("You don't have enough points")
                .color(Colour::ORANGE)
                .description("You need to win someone to get points")
                .footer(|f| {
                    f.text(format!("Request author name is {}", msg.author.name));

                    f
                });
        }
        else {
            sqlx::query!("UPDATE users SET balance = (balance - 1), strong = (strong + 1) WHERE id = $1;",
                msg.author.id.to_string())
                .execute(&pool)
                .await?;

            embed
                .title("Yeah!!!!")
                .color(Colour::ORANGE)
                .description("You have successfully upgraded your astrodolph. (strong tree)")
                .footer(|f| {
                    f.text(format!("Request author name is {}", msg.author.name));

                    f
                });
        }

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    } else {
        embed
            .title("who")
            .color(Colour::ORANGE)
            .description("You don't have your own astrodolph...")
            .footer(|f| {
                f.text(format!("Request author name is {}", msg.author.name));

                f
            });

        msg.channel_id.send_message(&ctx.http, |smv| {
            smv.content("").embed(|em| {
                em.0 = embed.0;
                em
            });
            smv
        }).await.unwrap();
    }
    Ok(())
}