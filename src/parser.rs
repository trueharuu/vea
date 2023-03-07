use crate::ast::*;
use crate::lexer::Span;
use crate::literal::Literal;
use crate::token::Token::{self, *};
use plex::parser;
macro_rules! b {
    [$T:expr] => {
        Box::new($T)
    };
}
parser! {
    fn parse_(Token, Span);

    // combine two spans
    (a, b) {
        Span(a.0, b.1)
    }

    program: Program {
        statements[s] => Program { stmts: s },
    }


    statements: Vec<Expr> {
      => vec![],
      statements[mut st] statement[e] => {
        st.push(e);
        st
      }
    }

    statement: Expr {
      Let Ident(t) Eq expr[a] Semi => Expr(span!(), Node::Let(t, Box::new(a))),
      // block[b] => b,
      expr[a] Semi => a,
    }



    expr: Expr {
      Print unary[b] => Expr(span!(), Node::Print(b![b])),
      Typeof unary[b] => Expr(span!(), Node::Typeof(b![b])),
      unary[b] => b,
      // block[b] => b,
    }

    // block: Expr {
    //   LeftBrace statements[st] RightBrace => Expr(span!(), Node::Block(st, None)),
    //   LeftBrace expr[st] RightBrace => Expr(span!(), Node::Block(Vec::new(), Some(Box::new(st)))),
    // }

    unary: Expr {
      Bang factor[b] => Expr(span!(), Node::Inv(b![b])),
      Minus factor[b] => Expr(span!(), Node::Neg(b![b])),
      Not factor[b] => Expr(span!(), Node::Not(b![b])),
      factor[a] => a,
    }

    factor: Expr {
      factor[a] Star term[b] => Expr(span!(), Node::Mul(b![a], b![b])),
      factor[a] Slash term[b] => Expr(span!(), Node::Div(b![a], b![b])),
      factor[a] Percent term[b] => Expr(span!(), Node::Rem(b![a], b![b])),
      term[a] => a,
    }

    term: Expr {
      term[a] Plus shift[b] => Expr(span!(), Node::Add(b![a], b![b])),
      term[a] Minus shift[b] => Expr(span!(), Node::Sub(b![a], b![b])),
      shift[a] => a,
    }

    shift: Expr {
      shift[a] Shl and[b] => Expr(span!(), Node::Shl(b![a], b![b])),
      shift[a] Shr and[b] => Expr(span!(), Node::Shr(b![a], b![b])),
      and[a] => a,
    }

    and: Expr {
      and[a] And xor[b] => Expr(span!(), Node::And(b![a], b![b])),
      xor[a] => a,
    }

    xor: Expr {
      xor[a] Xor or[b] => Expr(span!(), Node::Xor(b![a], b![b])),
      or[a] => a,
    }

    or: Expr {
      or[a] Or cmp[b] => Expr(span!(), Node::Or(b![a], b![b])),
      cmp[a] => a,
    }

    cmp: Expr {
      cmp[a] EqEq atom[b] => Expr(span!(), Node::Eq(b![a], b![b])),
      cmp[a] Ne atom[b] => Expr(span!(), Node::Ne(b![a], b![b])),
      cmp[a] Gt atom[b] => Expr(span!(), Node::Gt(b![a], b![b])),
      cmp[a] Lt atom[b] => Expr(span!(), Node::Lt(b![a], b![b])),
      cmp[a] Ge atom[b] => Expr(span!(), Node::Ge(b![a], b![b])),
      cmp[a] Le atom[b] => Expr(span!(), Node::Le(b![a], b![b])),

      atom[a] => a,
    }

    atom: Expr {
      Ident(i) => Expr(span!(), Node::Var(i),),
      Integer(i) => Expr(span!(), Node::Literal(Literal::Integer(i))),
      String(i) => Expr(span!(), Node::Literal(Literal::String(i))),
      True => Expr(span!(), Node::Literal(Literal::Boolean(true))),
      False => Expr(span!(), Node::Literal(Literal::Boolean(false))),
      Bang => Expr(span!(), Node::Literal(Literal::Never)),

      LeftParen expr[a] RightParen => a
    }
}

type Err = (Option<(Token, Span)>, &'static str);

pub fn parse<I: Iterator<Item = (Token, Span)>>(i: I) -> Result<Program, Err> {
    parse_(i)
}
