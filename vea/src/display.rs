use std::fmt::Display;

use crate::{ast::Expr, lexer::Token, literal::Literal};

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "&".to_string(),
                Self::AndEq => "&=".to_string(),
                Self::Bang => "!".to_string(),
                Self::Caret => "^".to_string(),
                Self::CaretEq => "^=".to_string(),
                Self::Comma => ",".to_string(),
                Self::Else => "else".to_string(),
                Self::Eq => "=".to_string(),
                Self::EqEq => "==".to_string(),
                Self::Error(..) => String::new(),
                Self::False => "false".to_string(),
                Self::Fn => "fn".to_string(),
                Self::For => "for".to_string(),
                Self::Ge => ">=".to_string(),
                Self::Gt => ">".to_string(),
                Self::Ident(t) => (*t).to_string(),
                Self::If => "if".to_string(),
                Self::Le => "<=".to_string(),
                Self::LeftBrace => "{".to_string(),
                Self::LeftBracket => "[".to_string(),
                Self::LeftParen => "(".to_string(),
                Self::Let => "let".to_string(),
                Self::Lt => "<".to_string(),
                Self::Minus => "-".to_string(),
                Self::MinusEq => "-=".to_string(),
                Self::Ne => "!=".to_string(),
                Self::Number(n) => n.to_string(),
                Self::Percent => "%".to_string(),
                Self::PercentEq => "%=".to_string(),
                Self::Pipe => "|".to_string(),
                Self::PipeEq => "|=".to_string(),
                Self::Plus => "+".to_string(),
                Self::PlusEq => "++".to_string(),
                Self::Print => "print".to_string(),
                Self::Question => "?".to_string(),
                Self::Quote => "'".to_string(),
                Self::Return => "return".to_string(),
                Self::RightBrace => "}".to_string(),
                Self::RightBracket => "]".to_string(),
                Self::RightParen => ")".to_string(),
                Self::Semi => ";".to_string(),
                Self::Set => "set".to_string(),
                Self::Shl => "<<".to_string(),
                Self::ShlEq => "<<=".to_string(),
                Self::Shr => ">>".to_string(),
                Self::ShrEq => ">>=".to_string(),
                Self::Slash => "/".to_string(),
                Self::SlashEq => "/=".to_string(),
                Self::Star => "*".to_string(),
                Self::StarEq => "*=".to_string(),
                Self::String(v) => format!("'{v}'"),
                Self::Struct => "struct".to_string(),
                Self::Tilde => "~".to_string(),
                Self::True => "true".to_string(),
                Self::Underscore => "_".to_string(),
                Self::While => "while".to_string(),
                Self::Colon => ":".to_string(),
                Self::DoubleColon => "::".to_string(),
                Self::Period => ".".to_string(),
            }
        )
    }
}

impl<'a> Expr<'a> {
    fn disp(&self, depth: usize) -> String {
        let mut m = String::from("\t").repeat(depth);

        let x = match self {
            Self::Access { ident } => ident.0.to_string(),
            Self::Print {
                left_paren,
                print_token,
                right_paren,
                semi_token,
                value,
            } => format!("{print_token}{left_paren}{value}{right_paren}{semi_token}"),
            Self::Return {
                return_token,
                value,
                semi_token,
            } => format!("{return_token} {value}{semi_token}"),
            Self::Literal { value } => match value {
                Literal::Fn(name, ..) => name.0.to_string(),
                Literal::Object(values) => {
                    format!(
                        "struct {{\n{}\n}}",
                        values
                            .iter()
                            .map(|(k, v)| {
                                match v.borrow().clone() {
                                    Literal::Fn(name, args, body) => format!(
                                        "{}fn {name}({}) {body}",
                                        "\t".repeat(depth + 1),
                                        args.iter()
                                            .map(ToString::to_string)
                                            .collect::<Vec<_>>()
                                            .join("\n")
                                    ),
                                    c => format!(
                                        "{}let {k} = {};",
                                        "\t".repeat(depth + 1),
                                        Self::Literal { value: c }.disp(depth + 1)
                                    ),
                                }
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
                Literal::Set(values) => {
                    format!(
                        "set {{\n{}\n}}",
                        values
                            .iter()
                            .map(|x| Expr::Literal {
                                value: x.borrow().clone()
                            }
                            .disp(depth + 1))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                }
                c => c.to_string(),
            },
            Self::Object {
                struct_token,
                left_brace,
                exprs,
                right_brace,
            } => format!(
                "{struct_token} {left_brace}\n{}\n{right_brace}",
                exprs
                    .iter()
                    .map(|x| x.0.disp(depth + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Self::Set {
                set_token,
                left_brace,
                exprs,
                right_brace,
            } => format!(
                "{set_token} {left_brace}\n{}\n{right_brace}",
                exprs
                    .iter()
                    .map(|x| x.0.disp(depth + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Self::Group {
                left_paren,
                expr,
                right_paren,
            } => format!("{left_paren}{expr}{right_paren}"),
            Self::Block {
                left_brace,
                exprs,
                right_brace,
            } => format!(
                "{left_brace}\n{}\n{right_brace}",
                exprs
                    .iter()
                    .map(|x| x.0.disp(depth + 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            Self::If {
                if_token,
                condition,
                then,
                else_token,
                other,
            } => {
                let mut m = format!("{if_token} ({condition}) {then}");

                if let Some(e) = else_token {
                    m += &e.to_string();
                }

                if let Some(o) = other {
                    m += &o.to_string();
                }

                m
            }
            Self::While {
                while_token,
                condition,
                then,
            } => format!("{while_token} ({condition}) {then}"),
            Self::FnDecl {
                fn_token,
                name,
                left_paren,
                arguments,
                right_paren,
                block,
            } => format!(
                "{fn_token} {name}{left_paren}{}{right_paren} {block}",
                arguments
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::FnCall {
                access,
                left_paren,
                arguments,
                right_paren,
            } => format!(
                "{access}{left_paren}{}{right_paren}",
                arguments
                    .iter()
                    .map(|x| x.0.disp(depth))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Let {
                let_token,
                ident,
                eq_token,
                expr,
                semi_token,
            } => format!("{let_token} {ident} {eq_token} {expr}{semi_token}"),
            Self::Assign {
                ident,
                eq_token,
                expr,
                semi_token,
            } => format!("{ident} {eq_token} {expr}{semi_token}"),
            Self::AddAssign {
                ident,
                plus_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {plus_eq_token} {expr}{semi_token}"),
            Self::SubAssign {
                ident,
                minus_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {minus_eq_token} {expr}{semi_token}"),
            Self::MulAssign {
                ident,
                star_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {star_eq_token} {expr}{semi_token}"),
            Self::DivAssign {
                ident,
                slash_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {slash_eq_token} {expr}{semi_token}"),
            Self::RemAssign {
                ident,
                percent_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {percent_eq_token} {expr}{semi_token}"),
            Self::ShlAssign {
                ident,
                shl_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {shl_eq_token} {expr}{semi_token}"),
            Self::ShrAssign {
                ident,
                shr_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {shr_eq_token} {expr}{semi_token}"),
            Self::AndAssign {
                ident,
                and_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {and_eq_token} {expr}{semi_token}"),
            Self::XorAssign {
                ident,
                caret_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {caret_eq_token} {expr}{semi_token}"),
            Self::OrAssign {
                ident,
                pipe_eq_token,
                expr,
                semi_token,
            } => format!("{ident} {pipe_eq_token} {expr}{semi_token}"),
            Self::Not { bang_token, expr } => format!("{bang_token}{expr}"),
            Self::Neg { minus_token, expr } => format!("{minus_token}{expr}"),

            Self::Add {
                lhs,
                plus_token,
                rhs,
            } => format!("{lhs} {plus_token} {rhs}"),
            Self::Sub {
                lhs,
                minus_token,
                rhs,
            } => format!("{lhs} {minus_token} {rhs}"),
            Self::Mul {
                lhs,
                star_token,
                rhs,
            } => format!("{lhs} {star_token} {rhs}"),
            Self::Div {
                lhs,
                slash_token,
                rhs,
            } => format!("{lhs} {slash_token} {rhs}"),
            Self::Rem {
                lhs,
                percent_token,
                rhs,
            } => format!("{lhs} {percent_token} {rhs}"),
            Self::Shl {
                lhs,
                shl_token,
                rhs,
            } => format!("{lhs} {shl_token} {rhs}"),
            Self::Shr {
                lhs,
                shr_token,
                rhs,
            } => format!("{lhs} {shr_token} {rhs}"),
            Self::And {
                lhs,
                and_token,
                rhs,
            } => format!("{lhs} {and_token} {rhs}"),
            Self::Xor {
                lhs,
                caret_token,
                rhs,
            } => format!("{lhs} {caret_token} {rhs}"),
            Self::Or {
                lhs,
                pipe_token,
                rhs,
            } => format!("{lhs} {pipe_token} {rhs}"),
            Self::Eq {
                lhs,
                eqeq_token,
                rhs,
            } => format!("{lhs} {eqeq_token} {rhs}"),
            Self::Ne { lhs, ne_token, rhs } => format!("{lhs} {ne_token} {rhs}"),
            Self::Gt { lhs, gt_token, rhs } => format!("{lhs} {gt_token} {rhs}"),
            Self::Ge { lhs, ge_token, rhs } => format!("{lhs} {ge_token} {rhs}"),
            Self::Lt { lhs, lt_token, rhs } => format!("{lhs} {lt_token} {rhs}"),
            Self::Le { lhs, le_token, rhs } => format!("{lhs} {le_token} {rhs}"),

            Self::Chain { parent, child } => format!("{parent}:{child}"),

            Self::None => String::from("_"),
            Self::Error(..) => String::from("@"),
        };

        m += &x;
        m
    }
}
impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.disp(0);

        write!(f, "{x}")
    }
}
