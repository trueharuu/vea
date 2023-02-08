use crate::ast::*;
use crate::lexer::Token::*;
use crate::lexer::*;
use plex::parser;
parser! {
    fn parse_(Token, Span);

    // combine two spans
    (a, b) {
        Span(a.0, b.1)
    }

    program: Program {
        statements[s] => Program { stmts: s }
    }

    statements: Vec<Expr> {
        => vec![],
        statements[mut st] assign[e] Semi => {
            st.push(e);
            st
        }
    }

    assign: Expr {
        Print assign[a] => Expr {
            span: span!(),
            node: Node::Print(Box::new(a)),
        },
        Typeof assign[a] => Expr {
            span: span!(),
            node: Node::Typeof(Box::new(a)),
        },
        Ident(var) Equals assign[rhs] => Expr {
            span: span!(),
            node: Node::Assign(var, Box::new(rhs)),
        },
        term[t] => t,
    }

    term: Expr {
        term[lhs] Plus fact[rhs] => Expr {
            span: span!(),
            node: Node::Add(Box::new(lhs), Box::new(rhs)),
        },
        term[lhs] Minus fact[rhs] => Expr {
            span: span!(),
            node: Node::Sub(Box::new(lhs), Box::new(rhs)),
        },
        fact[x] => x
    }

    cmp: Expr {
        cmp[]
    }

    fact: Expr {
        fact[lhs] Star atom[rhs] => Expr {
            span: span!(),
            node: Node::Mul(Box::new(lhs), Box::new(rhs)),
        },
        fact[lhs] Slash atom[rhs] => Expr {
            span: span!(),
            node: Node::Div(Box::new(lhs), Box::new(rhs)),
        },
        atom[x] => x
    }

    atom: Expr {
        // round brackets to destructure tokens
        Ident(i) => Expr {
            span: span!(),
            node: Node::Var(i),
        },
        Integer(i) => Expr {
            span: span!(),
            node: Node::Literal(Literal::Integer(i)),
        },
        String(i) => Expr {
            span: span!(),
            node: Node::Literal(Literal::String(i))
        },
        True => Expr {
            span: span!(),
            node: Node::Literal(Literal::Boolean(true))
        },
        False => Expr {
            span: span!(),
            node: Node::Literal(Literal::Boolean(false))
        },
        LeftParen assign[a] RightParen => a,
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I,
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}
