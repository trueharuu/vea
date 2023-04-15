#![warn(clippy::all)]

use ariadne::sources;
use ariadne::Color;

use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use chumsky::prelude::Input;
use chumsky::Parser;
use lexer::lexer;

use crate::interpreter::exec;
use crate::parser::parser;

mod ast;
mod common;
pub mod interpreter;
mod lexer;
mod parser;
fn main() {
    let src = "let x = 1 == 1;";
    dbg!(src);

    let oe = lexer().parse(src).into_output_errors();

    oe.1.clone()
        .into_iter()
        .map(|x: _| x.map_token(|c: _| c.to_string()))
        .for_each(|x: _| {
            Report::build(ReportKind::Error, "test.vea", x.span().start)
                .with_message(x.to_string())
                .with_label(
                    Label::new(("test.vea", x.span().into_range()))
                        .with_message(x.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .eprint(sources([("test.vea", src)]))
                .unwrap();
        });

    if let Some(a) = oe.0.clone() {
        let p = parser()
            .parse(a.spanned((src.len()..src.len()).into()))
            .into_output_errors();

        p.1.clone()
            .into_iter()
            .map(|x: _| x.map_token(|c: _| format!("{c:?}")))
            .for_each(|x: _| {
                Report::build(ReportKind::Error, "test.vea", x.span().start)
                    .with_message(x.to_string())
                    .with_label(
                        Label::new(("test.vea", x.span().into_range()))
                            .with_message(x.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(sources([("test.vea", src)]))
                    .unwrap();
            });

        if let Some(p) = p.0.clone() {
            let e = exec(p);

            if let Err((x, y)) = &e {
                // e.into_iter().for_each(|(x, y)| {
                Report::build(ReportKind::Error, "test.vea", y.start)
                    .with_message(x.clone())
                    .with_label(
                        Label::new(("test.vea", y.into_range()))
                            .with_message(x)
                            .with_color(Color::Blue),
                    )
                    .finish()
                    .eprint(sources([("test.vea", src)]))
                    .unwrap();
                // });
            }

            println!("{}", e.unwrap().stdout);
        }
    }
}
