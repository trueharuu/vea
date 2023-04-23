use std::time::Duration;

use poise::builtins;
use poise::serenity_prelude::GatewayIntents;
use poise::CodeBlock;
use poise::EditTracker;

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            allowed_mentions: None,
            commands: vec![lex(), ast(), vea()],

            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".to_owned()),
                // 10 minutes
                edit_tracker: Some(EditTracker::for_timespan(Duration::from_secs(600))),
                ..Default::default()
            },

            reply_callback: Some(|_, y| {
                y.reply(true)
                    .allowed_mentions(|x| x.users::<u64>([]).roles::<u64>([]).replied_user(false));
            }),

            ..Default::default()
        })
        .token(dotenv::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(GatewayIntents::all())
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                // // clear guild commands

                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result = std::result::Result<(), Error>;

#[poise::command(prefix_command)]
async fn lex(context: Context<'_>, src: CodeBlock) -> Result {
    let l = vea::lex(&src.code);

    if !l.1.is_empty() {
        context
            .say(format!("lexing errors: ```ansi\n{}\n```", l.1))
            .await?;
    }

    if let Some(t) = l.0 {
        context
            .say(format!(
                "```rs\n{:#?}\n```",
                t.into_iter().collect::<Vec<_>>()
            ))
            .await?;
    }

    Ok(())
}

#[poise::command(prefix_command)]
async fn ast(context: Context<'_>, src: CodeBlock) -> Result {
    let l = vea::lex(&src.code);

    if !l.1.is_empty() {
        context
            .say(format!("lexing errors: ```ansi\n{}\n```", l.1))
            .await?;
    }

    if let Some(t) = l.0 {
        let x = vea::parse(&src.code, t);

        if !x.1.is_empty() {
            context
                .say(format!("parsing errors: ```ansi\n{}\n```", x.1))
                .await?;
        }

        if let Some(p) = x.0 {
            context.say(format!("```rs\n{p:#?}\n```")).await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command)]
async fn vea(context: Context<'_>, src: CodeBlock) -> Result {
    let l = vea::lex(&src.code);

    if !l.1.is_empty() {
        context
            .say(format!("lexing errors: ```ansi\n{}\n```", l.1))
            .await?;
    }

    if let Some(t) = l.0 {
        let x = vea::parse(&src.code, t.clone());

        dbg!(&x);

        if !x.1.is_empty() {
            context
                .say(format!("parsing errors: ```ansi\n{}\n```", x.1))
                .await?;
        }

        if let Some(p) = x.0 {
            let m = vea::interp(&src.code, t, p);
            if !m.is_empty() {
                context.say(format!("```ansi\n{m}\n```")).await?;
            }
        }
    }

    Ok(())
}
