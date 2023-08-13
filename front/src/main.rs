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
            commands: vec![lex(), ast(), exec(), ri()],

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
async fn lex(context: Context<'_>, esrc: Option<CodeBlock>) -> Result {
    let src = if let Some(e) = esrc {
        e
    } else {
        context.say("code?".to_owned()).await?;
        return Ok(());
    };

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
async fn ast(context: Context<'_>, esrc: Option<CodeBlock>) -> Result {
    if let Some(e) = esrc {
        let x = v_ast(&e.code);
        context.say(x).await?;
    } else {
        context.say("code?".to_owned()).await?;
    };

    Ok(())
}

fn v_ast(c: &str) -> String {
    let mut m = String::new();
    let l = vea::lex(c);

    if !l.1.is_empty() {
        m += &format!("lexing errors: ```ansi\n{}\n```", l.1);
    }

    if let Some(t) = l.0 {
        let x = vea::parse(c, &t);

        if !x.1.is_empty() {
            m += &format!("parsing errors: ```ansi\n{}\n```", x.1);
        }

        if let Some(p) = x.0 {
            m += &format!("```rs\n{p:#?}\n```");
        }
    }

    m
}

#[poise::command(prefix_command)]
async fn exec(context: Context<'_>, esrc: Option<CodeBlock>) -> Result {
    if let Some(e) = esrc {
        let x = v_exec(&e.code);
        context.say(x).await?;
    } else {
        context.say("code?".to_owned()).await?;
        return Ok(());
    };

    Ok(())
}

fn v_exec(c: &str) -> String {
    let mut z = String::new();
    let l = vea::lex(c);

    if !l.1.is_empty() {
        z += &format!("lexing errors: ```ansi\n{}\n```", l.1);
    }

    if let Some(t) = l.0 {
        let x = vea::parse(c, &t.clone());

        if !x.1.is_empty() {
            z += &format!("parsing errors: ```ansi\n{}\n```", x.1);
        }

        if let Some(p) = x.0 {
            let m = vea::interp(c, &t, p);
            if !m.is_empty() {
                z += &format!("```ansi\n{m}\n```");
            }
        }
    }

    z
}

fn v_ri(c: &str) -> String {
    let mut m = String::new();
    let l = vea::lex(c);

    if !l.1.is_empty() {
        m += &format!("lexing errors: ```ansi\n{}\n```", l.1);
    }

    if let Some(t) = l.0 {
        let x = vea::parse(c, &t);

        if !x.1.is_empty() {
            m += &format!("parsing errors: ```ansi\n{}\n```", x.1);
        }

        if let Some(p) = x.0 {
            m += &format!(
                "```rs\n{}\n```",
                p.into_iter()
                    .map(|x| x.0.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    m
}

#[poise::command(prefix_command)]
async fn ri(context: Context<'_>, esrc: Option<CodeBlock>) -> Result {
    if let Some(e) = esrc {
        let x = v_ri(&e.code);
        context.say(x).await?;
    } else {
        context.say("code?".to_owned()).await?;
        return Ok(());
    };

    Ok(())
}
