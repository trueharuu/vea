use crate::ast::*;
use crate::lexer::Token::*;
use crate::lexer::{ Span, Token };
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

        Env get[g] => Expr {
          span: span!(),
          node: Node::InnerEnv(Box::new(g))
        },

        Env Ident(var) => Expr {
          span: span!(),
          node: Node::Env(var),
        },

        get[g] Equals assign[rhs] => Expr {
          span: span!(),
          node: Node::Set(Box::new(g), Box::new(rhs))
        },

        get[t] => t,

        // Fn Ident(name) LeftParen list[a] RightParen LeftBrace statements[s] RightBrace => Expr {
        //   span: span!(),
        //   node: Node::Fn(name, Box::new(a), s)
        // },

        cmp[t] => t,
    }

    get: Expr {
      Ident(obj) Dot get[prop] => {
        let node = prop.node;
        if let Node::Get(n, v) = node {
        Expr {
          span: span!(),
          node: Node::Get(obj, [vec![n], v].concat())
        } } else { panic!("invalid get accessor")}
    },

      Ident(obj) Dot Ident(prop) => Expr {
        span: span!(),
        node: Node::Get(obj, vec![prop])
      }
    }

    list: Expr {
      cmp[lhs] Comma list[rhs] => Expr {
        span: span!(),
        node: Node::Pair(Box::new(lhs), Box::new(rhs))
      },

      cmp[lhs] => lhs
    }

    
    cmp: Expr {
      term[lhs] Eq term[rhs] => Expr {
        span: span!(),
        node: Node::Eq(Box::new(lhs), Box::new(rhs)),
      },
      term[lhs] Ne term[rhs] => Expr {
        span: span!(),
        node: Node::Ne(Box::new(lhs), Box::new(rhs)),
      },
      term[lhs] Gt term[rhs] => Expr {
        span: span!(),
        node: Node::Gt(Box::new(lhs), Box::new(rhs)),
      },
      term[lhs] Lt term[rhs] => Expr {
        span: span!(),
        node: Node::Lt(Box::new(lhs), Box::new(rhs)),
      },
      term[lhs] Ge term[rhs] => Expr {
        span: span!(),
        node: Node::Ge(Box::new(lhs), Box::new(rhs)),
      },
      term[lhs] Le term[rhs] => Expr {
        span: span!(),
        node: Node::Le(Box::new(lhs), Box::new(rhs)),
      },
      term[x] => x
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
        Bang => Expr {
          span: span!(),
          node: Node::Literal(Literal::None)
        },
        LeftParen assign[a] RightParen => a,
    }
}

pub fn parse<I: Iterator<Item = (Token, Span)>>(
    i: I
) -> Result<Program, (Option<(Token, Span)>, &'static str)> {
    parse_(i)
}