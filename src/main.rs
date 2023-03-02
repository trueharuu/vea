#![allow(unused_braces)]
#![feature(
    proc_macro_hygiene,
    iter_next_chunk,
    let_chains,
    if_let_guard,
    is_some_and,
    result_option_inspect
)]
use std::collections::HashMap;

use interpreter::interp;
use lexer::Lexer;

use parser::parse;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{Args, CommandResult};
use serenity::framework::StandardFramework;
use serenity::http::CacheHttp;
use serenity::model::prelude::Message;
use serenity::prelude::*;
use serenity::{async_trait, framework::standard::macros::command};

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod tools;

#[group]
#[commands(eval)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "NzYwMTQzNjE1MTI0NDM5MDQw.Gv5Sqp.-Ff1jKvi8uG-8m_5mkJj50waVOXMeK25r1rM2I";
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let value = args.rest();

    let lexed = Lexer::new(value);
    let parsed = parse(lexed);
    if let Ok(mut p) = parsed {
        let mut n = HashMap::new();
        let mut s = String::new();
        let interp = interp(&mut p, &mut n, &mut s);
        if let Ok(i) = interp {
            msg.reply(ctx.http(), format!("```rs\n{i}\n```")).await?;
        } else if let Err(i) = interp {
            msg.reply(ctx.http(), format!("```rs\nError at runtime:\n{}\n```", i))
                .await?;
        }
    } else if let Err(p) = parsed {
        msg.reply(
            ctx.http(),
            format!(
                "```rs\nError while parsing text:\n\t{}\nToken Tree:\n\t{:?}\n```",
                p.1, p.0
            ),
        )
        .await?;
    }

    Ok(())
}
