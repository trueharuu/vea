#![allow(unused)]
#![feature(is_some_and, let_chains, option_result_contains)]

use lox::Lox;

pub mod ast;
pub mod ast_printer;
pub mod interpreter;
pub mod literal;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod token;
pub mod env;
fn main() {
    let r =
        r#"var a = "global a";
    var b = "global b";
    var c = "global c";
    {
      var a = "outer a";
      var b = "outer b";
      {
        var a = "inner a";
        print a;
        print b;
        print c;
      }
      print a;
      print b;
      print c;
    }
    print a;
    print b;
    print c;"#;
    Lox::new().run(r.to_string())
}