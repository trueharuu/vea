#![feature(
    proc_macro_hygiene,
    iter_next_chunk,
    let_chains,
    if_let_guard,
    is_some_and,
    result_option_inspect,
    result_flattening
)]
#![warn(clippy::all)]
#![allow(unused_braces, clippy::redundant_closure_call, clippy::ptr_arg)]

use std::cell::RefCell;
use std::rc::Rc;

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod literal;
pub mod parser;
pub mod token;
pub mod tools;

use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{async_trait, framework::standard::Args};
use serenity::{
    framework::standard::macros::{command, group},
    http::CacheHttp,
};

#[group]
#[commands(eval, lex)]
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
    let token = dotenv::var("TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {why:?}");
    }
}

#[command]
async fn eval(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let src = args.rest();
    msg.reply(ctx.http(), bump(src)).await?;
    Ok(())
}

#[command]
async fn lex(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    println!("lex!");
    let src = args.rest();
    let tokens = lexer::Lexer::new(src).collect::<Vec<_>>();

    for i in tokens.clone() {
        if let Err(e) = i.0 {
            msg.reply(
                ctx.http(),
                format!(":warning: lexing error:\n```rs\n{e} at chars {:?}```", i.1),
            )
            .await?;
            return Ok(());
        }
    }

    msg.reply(
        ctx.http(),
        format!(
            "output:\n```rs\n{}\n```",
            tokens
                .into_iter()
                .map(|(c, p)| format!("{: >3}..{: <3} {:?}", p.0, p.1, c.unwrap()))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    )
    .await?;
    Ok(())
}

fn bump(src: &str) -> String {
    let tokens = lexer::Lexer::new(src).collect::<Vec<_>>();
    println!("recieved tokens: {tokens:?}");
    for i in tokens.clone() {
        if let Err(e) = i.0 {
            return format!(":warning: lexing error:\n```rs\n{e} at chars {:?}```", i.1);
        }
    }
    let exprs = parser::parse(tokens.into_iter().map(|(x, y)| (x.unwrap(), y)));

    if let Err(e) = exprs {
        format!(
            ":warning: parsing error:\n```rs\n{}\n```\ntoken info:\n```rs\n{}\n```",
            e.1,
            if let Some((a, b)) = e.0 {
                format!("{a:?}\n\t= span: {b:?}")
            } else {
                "None".to_string()
            }
        )
    } else {
        let mut p = exprs.unwrap();

        let mut env = Rc::new(RefCell::new(interpreter::Env::new()));
        let mut stdout = String::new();

        let i = interpreter::interp(&mut p, &mut env, &mut stdout);

        if let Err(e) = i {
            format!(":warning: runtime error:\n```rs\n{e}\n```")
        } else {
            format!("output:\n```rs\n{}\n```", i.unwrap())
        }
    }
}
