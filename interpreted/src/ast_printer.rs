use crate::{
    ast::expr::{Expr, ExprVisitor},
    literal::Literal,
};

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Binary(left, operator, right) = expr {
            self.parenthesize(
                operator.lexeme.to_string(),
                vec![*left.clone(), *right.clone()],
            )
        } else {
            unreachable!();
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Grouping(source) = expr {
            self.parenthesize("group".to_string(), vec![*source.clone()])
        } else {
            unreachable!();
        }
    }

    fn visit_literal_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Literal(source) = expr {
            return source.to_string();
        } else {
            unreachable!();
        }
    }

    fn visit_unary_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Unary(operator, right) = expr {
            self.parenthesize(operator.lexeme.to_string(), vec![*right.clone()])
        } else {
            unreachable!();
        }
    }

    fn visit_assign_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Assign(name, value) = expr {
            self.parenthesize(
                "call".to_string(),
                vec![
                    Expr::Literal(Literal::String(name.lexeme.to_string())),
                    *value.clone(),
                ],
            )
        } else {
            unreachable!();
        }
    }

    fn visit_call_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Call(callee, _, arguments) = expr {
            self.parenthesize(
                "call ".to_string() + &callee.accept(self),
                arguments
                    .to_owned()
                    .iter()
                    .map(|x| *x.clone())
                    .collect::<Vec<Expr>>(),
            )
        } else {
            unreachable!();
        }
    }

    fn visit_get_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Get(source, name) = expr {
            self.parenthesize(
                "get".to_string(),
                vec![
                    *source.clone(),
                    Expr::Literal(Literal::String(name.lexeme.to_string())),
                ],
            )
        } else {
            unreachable!();
        }
    }

    fn visit_logical_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Logical(left, operator, right) = expr {
            self.parenthesize(
                operator.lexeme.to_string(),
                vec![*left.clone(), *right.clone()],
            )
        } else {
            unreachable!();
        }
    }

    fn visit_set_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Set(source, name, value) = expr {
            self.parenthesize(
                "set".to_string(),
                vec![
                    *source.clone(),
                    Expr::Literal(Literal::String(name.lexeme.to_string())),
                    *value.clone(),
                ],
            )
        } else {
            unreachable!();
        }
    }

    fn visit_super_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Super(keyword, method) = expr {
            self.parenthesize(
                "super ".to_string() + &keyword.lexeme + " " + &method.lexeme,
                vec![],
            )
        } else {
            unreachable!()
        }
    }

    fn visit_this_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::This(_) = expr {
            self.parenthesize("this".to_string(), vec![])
        } else {
            unreachable!()
        }
    }

    fn visit_variable_expr(&self, expr: &crate::ast::expr::Expr) -> String {
        if let Expr::Variable(name) = expr {
            self.parenthesize(
                "var".to_string(),
                vec![Expr::Literal(Literal::String(name.lexeme.to_string()))],
            )
        } else {
            unreachable!();
        }
    }
}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&self, name: String, exprs: Vec<Expr>) -> String {
        let mut builder = String::new();

        builder += "(";
        builder += &name;
        for expr in exprs {
            builder += " ";
            builder += &expr.accept(self)
        }

        builder += ")";

        builder
    }
}
