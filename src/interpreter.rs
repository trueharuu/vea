use std::{cell::RefCell, error::Error, fmt::Display, ops::BitOr, rc::Rc};

use crate::{
    ast::{
        expr::{Expr, ExprVisitor},
        statement::{Stmt, StmtVisitor},
    },
    callable::Callable,
    env::Env,
    everest::Everest,
    literal::Literal,
    token::{Token, TokenKind},
};

#[derive(Clone)]
pub struct Interpreter {
    pub lox: Box<Everest>,
    pub env: Rc<RefCell<Env>>,
    pub globals: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new(lox: Box<Everest>) -> Self {
        let globals = Rc::new(RefCell::new(Env::new()));

        Self {
            lox,
            env: Rc::new(RefCell::new(Env::new())),
            globals,
        }
    }

    pub fn eval(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    pub fn exec(&mut self, stmt: &mut Stmt) {
        stmt.accept(self)
    }

    pub fn collapse(&self, expr: &Value) -> Literal {
        match expr {
            Value::Literal(x) => x.clone(),
            Value::Expr(e) => {
                let x = self.eval(e);
                self.collapse(&x.unwrap())
            }
        }
    }

    fn check_number_operand(&self, operator: Token, operand: Literal) -> Result<(), RuntimeError> {
        if !matches!(operand, Literal::Number(_)) {
            Err(RuntimeError::new(
                operator.clone(),
                format!("operand of `{}x` must be of type number", operator.clone()),
            ))
        } else {
            Ok(())
        }
    }

    fn check_number_operands(
        &self,
        operator: Token,
        left: Literal,
        right: Literal,
    ) -> Result<(), RuntimeError> {
        if !matches!(left, Literal::Number(_)) || !matches!(right, Literal::Number(_)) {
            Err(RuntimeError::new(
                operator.clone(),
                format!("operands of `x {} y` must be numbers", operator.clone()),
            ))
        } else {
            Ok(())
        }
    }

    pub fn interpret(&mut self, statements: Result<Vec<Stmt>, RuntimeError>) {
        if let Ok(v) = statements {
            for mut statement in v {
                self.exec(&mut statement);
            }
        } else {
            self.lox.runtime_err(statements.unwrap_err());
        }
    }

    fn stringify(&self, obj: Literal) -> String {
        obj.to_string()
    }

    pub fn exec_block(&mut self, statements: Vec<Stmt>, env: Env) -> () {
        let prev = self.env.clone();
        self.env = Rc::new(RefCell::new(env));

        for mut s in statements {
            self.exec(&mut s);
        }

        self.env = prev.clone();
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Expr(Expr),
}

impl BitOr<&Value> for &Interpreter {
    type Output = Literal;
    fn bitor(self, rhs: &Value) -> Self::Output {
        self.collapse(&rhs)
    }
}

impl BitOr<&Value> for &&mut Interpreter {
    type Output = Literal;
    fn bitor(self, rhs: &Value) -> Self::Output {
        self.collapse(&rhs)
    }
}

impl ExprVisitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_literal_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Literal(x) = expr {
            Ok(Value::Literal(x.clone()))
        } else {
            unreachable!();
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Grouping(x) = expr {
            self.eval(x)
        } else {
            unreachable!();
        }
    }

    fn visit_unary_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Unary(op, r) = expr {
            let r = self.eval(r);
            if let Ok(right) = r {
                match op.kind {
                    TokenKind::Minus => {
                        let ck = self.check_number_operand(op.clone(), self | &right);
                        ck.map(|_| Value::Literal(Literal::Number(-self.collapse(&right))))
                    }
                    TokenKind::Bang => Ok(Value::Literal(Literal::Boolean(!self.collapse(&right)))),
                    _ => unreachable!(),
                }
            } else {
                r
            }
        } else {
            unreachable!()
        }
    }

    fn visit_binary_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Binary(l, op, r) = expr {
            let l = self.eval(l);
            let r = self.eval(r);

            if let Ok(left) = l.clone() && let Ok(right) = r {
                match op.kind {
                    TokenKind::Minus => {
                        self.check_number_operands(op.clone(), self | &left, self | &right).map(|_|
                            Value::Literal(
                                Literal::Number(self.collapse(&left) - self.collapse(&right))
                            )
                        )
                    }
                    TokenKind::Slash => {
                        let ck = self.check_number_operands(
                            op.clone(),
                            self | &left,
                            self | &right
                        );
                        ck.map(|_|
                            Value::Literal(
                                Literal::Number(self.collapse(&left) / self.collapse(&right))
                            )
                        )
                    }
                    TokenKind::Star => {
                        let ck = self.check_number_operands(
                            op.clone(),
                            self | &left,
                            self | &right
                        );
                        ck.map(|_|
                            Value::Literal(
                                Literal::Number(self.collapse(&left) * self.collapse(&right))
                            )
                        )
                    }
                    TokenKind::Plus => if
                        let Literal::String(x) = self.collapse(&left) &&
                        let Literal::String(y) = self.collapse(&left)
                    {
                        Ok(Value::Literal(Literal::String(x + &y)))
                    } else if
                        let Literal::Number(x) = self | &left &&
                        let Literal::Number(y) = self | &right
                    {
                        Ok(Value::Literal(Literal::Number(x + y)))
                    } else {
                        Err(
                            RuntimeError::new(
                                op.clone(),
                                "operands of x + y must be of type (string | number)".to_string()
                            )
                        )
                    }

                    TokenKind::Gt => {
                        let ck = self.check_number_operands(
                            op.clone(),
                            self | &left,
                            self | &right
                        );
                        ck.map(|_| Value::Literal(Literal::Boolean(self | &left > self | &right)))
                    }
                    TokenKind::Ge => {
                        let ck = self.check_number_operands(
                            op.clone(),
                            self | &left,
                            self | &right
                        );
                        ck.map(|_| Value::Literal(Literal::Boolean(self | &left >= self | &right)))
                    }
                    TokenKind::Lt => {
                        let ck = self.check_number_operands(
                            op.clone(),
                            self | &left,
                            self | &right
                        );
                        ck.map(|_| Value::Literal(Literal::Boolean(self | &left < self | &right)))
                    }
                    TokenKind::Le => {
                        self.check_number_operands(op.clone(), self | &left, self | &right);
                        Ok(Value::Literal(Literal::Boolean(self | &left <= self | &right)))
                    }

                    TokenKind::Ne =>
                        Ok(Value::Literal(Literal::Boolean((self | &left) != (self | &right)))),
                    TokenKind::Eq =>
                        Ok(Value::Literal(Literal::Boolean((self | &left) == (self | &right)))),

                    _ => Ok(unreachable!()),
                }
            } else {
                l.and(r)
            }
        } else {
            unreachable!();
        }
    }

    fn visit_assign_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Assign(name, expr) = expr {
            let value = self.eval(&**expr);
            if let Ok(v) = value.clone() {
                self.env
                    .borrow_mut()
                    .assign(name.clone(), self.collapse(&v));
            }

            value
        } else {
            unreachable!();
        }
    }

    fn visit_call_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Call(callee, paren, arguments) = expr {
            let calli = self.eval(callee);

            if calli.is_err() {
                return calli;
            }

            let mut argv = Vec::new();

            for i in arguments {
                argv.push(*i.clone());
            }

            let raw = self.collapse(&calli.unwrap());

            if let Literal::Fn(_) = raw {
                raw.call(&mut self, argv.iter().map(|x| self.collapse(&self.eval(x).unwrap())).collect());
            }
            todo!();
        } else {
            unreachable!();
        }
    }

    fn visit_get_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_logical_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Logical(l, op, r) = expr {
            let left = self.eval(l);

            if left.is_err() {
                return left;
            }

            if op.kind == TokenKind::Or {
                if !!(self | left.as_ref().unwrap()) {
                    return left;
                }
            } else {
                if !(self | left.as_ref().unwrap()) {
                    return left;
                }
            }

            return self.eval(r);
        } else {
            unreachable!();
        }
    }

    fn visit_set_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        todo!()
    }

    fn visit_variable_expr(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        if let Expr::Variable(c) = expr {
            let rf = self.env.borrow().get(c.clone());
            if let Ok(r) = rf {
                Ok(Value::Literal(r))
            } else {
                Ok(Value::Literal(Literal::None))
            }
        } else {
            unreachable!();
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &crate::ast::statement::Stmt) -> () {
        if let Stmt::Expression(expr) = stmt {
            self.eval(expr);
        }
    }

    fn visit_print_stmt(&self, stmt: &Stmt) -> () {
        if let Stmt::Print(expr) = stmt {
            let value = self.eval(expr);
            match value {
                Ok(o) => println!("{}", self | &o),
                Err(e) => panic!("bad! {e}"),
            }
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Block(statements) = stmt {
            self.exec_block(
                statements.to_vec(),
                Env::with_parent(Box::new(self.env.clone().borrow_mut().to_owned())),
            );
        }
    }

    fn visit_class_stmt(&self, stmt: &Stmt) -> () {
        todo!()
    }

    fn visit_fn_stmt(&self, stmt: &Stmt) -> () {
        println!("making new fn");
        if let Stmt::Fn(name, params, body) = stmt {
            self.env
                .borrow_mut()
                .define(name.lexeme.clone(), Literal::Fn(Box::new(stmt.clone())))
        }
    }

    fn visit_if_stmt(&mut self, stmt: &mut Stmt) -> () {
        if let Stmt::If(cond, then, el) = stmt {
            let ev = self.eval(cond);
            match ev {
                Ok(e) => {
                    if !!self.collapse(&e) {
                        self.exec(then);
                    } else if let Some(se) = el {
                        self.exec(se)
                    }
                }
                Err(e) => panic!("bad! {e}"),
            }
        } else {
            unreachable!();
        }
    }

    fn visit_return_stmt(&self, stmt: &Stmt) -> () {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &Stmt) -> () {
        if let Stmt::Var(name, initializer) = stmt {
            let value = self.eval(initializer);
            if let Ok(r) = value {
                let v = &self | &r;
                self.env.borrow().define(name.lexeme.clone(), v);
            }
        }
    }

    fn visit_while_stmt(&mut self, stmt: &mut Stmt) -> () {
        if let Stmt::While(cond, body) = stmt {
            while let Ok(o) = self.eval(cond) && !!self.collapse(&o) {
                self.exec(&mut *body);
            }

            // let mut out = self.eval(cond);

            // if let Ok(o) = out {
            //     while !!(self.collapse(&o)) {
            //         self.exec(&mut *body);
            //     }
            // }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError(pub Token, pub String);

impl RuntimeError {
    pub fn new(token: Token, message: String) -> Self {
        Self(token, message)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for RuntimeError {}
