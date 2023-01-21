import { Everest } from './everest';
import type { Expr } from './expr';
import type { Interpreter } from './interpreter';
import { Stack } from './stack';
import type { Stmt } from './stmt';
import type { Token } from './token';

export class Resolver implements Expr.Visitor<void>, Stmt.Visitor<void> {
  private readonly interpreter: Interpreter;
  private readonly scopes: Stack<Map<string, boolean>> = new Stack();
  private current_fn: FnKind = FnKind.None;
  private current_class: ClassKind = ClassKind.None;

  constructor(interpreter: Interpreter) {
    this.interpreter = interpreter;
  }

  public resolve(statements: Array<Stmt>): void {
    for (const statement of statements) {
      this.resolve_stmt(statement);
    }
  }

  visit_block_stmt(stmt: Stmt.Block): void {
    this.begin_scope();
    this.resolve(stmt.statements);
    this.end_scope();
  }

  visit_class_stmt(stmt: Stmt.Class): void {
    const enclosing_class = this.current_class;
    this.current_class = ClassKind.Class;
    this.declare(stmt.name);
    this.define(stmt.name);

    if (
      stmt.superclass !== undefined &&
      stmt.name.lexeme === stmt.superclass.name.lexeme
    ) {
      Everest.error_with(stmt.superclass.name, 'class cannot inherit itself');
    }

    if (stmt.superclass !== undefined) {
      this.current_class = ClassKind.Subclass;
      this.resolve_expr(stmt.superclass);
    }

    if (stmt.superclass !== undefined) {
      this.begin_scope();
      this.scopes.peek().set('super', true);
    }

    this.begin_scope();
    this.scopes.peek().set('this', true);

    for (const method of stmt.methods) {
      let declaration = FnKind.Method;
      if (method.name.lexeme === 'init') {
        declaration = FnKind.Initializer;
      }

      this.resolve_fn(method, declaration);
    }

    this.end_scope();

    if (stmt.superclass !== undefined) {
      this.end_scope();
    }

    this.current_class = enclosing_class;
  }

  visit_expression_stmt(stmt: Stmt.Expression): void {
    this.resolve_expr(stmt.expression);
  }

  visit_fn_stmt(stmt: Stmt.Fn): void {
    this.declare(stmt.name);
    this.define(stmt.name);

    this.resolve_fn(stmt, FnKind.Fn);
  }

  visit_if_stmt(stmt: Stmt.If): void {
    this.resolve_expr(stmt.condition);
    this.resolve_stmt(stmt.then_branch);
    if (stmt.else_branch !== undefined) {
      this.resolve_stmt(stmt.else_branch);
    }
  }

  visit_print_stmt(stmt: Stmt.Print): void {
    this.resolve_expr(stmt.expression);
  }

  visit_return_stmt(stmt: Stmt.Return): void {
    if (this.current_fn === FnKind.None) {
      Everest.error_with(stmt.keyword, 'cannot return outside of fns');
    }

    if (stmt.value !== undefined) {
      if (this.current_fn === FnKind.Initializer) {
        Everest.error_with(stmt.keyword, 'cannot return in constructor');
      }

      this.resolve_expr(stmt.value);
    }
  }

  visit_var_stmt(stmt: Stmt.Var): void {
    this.declare(stmt.name);
    if (stmt.initializer !== undefined) {
      this.resolve_expr(stmt.initializer);
    }

    this.define(stmt.name);
  }

  visit_while_stmt(stmt: Stmt.While): void {
    this.resolve_expr(stmt.condition);
    this.resolve_stmt(stmt.body);
  }

  visit_assign_expr(expr: Expr.Assign): void {
    this.resolve_expr(expr.value);
    this.resolve_local(expr, expr.name);
  }

  visit_binary_expr(expr: Expr.Binary): void {
    this.resolve_expr(expr.left);
    this.resolve_expr(expr.right);
  }

  visit_call_expr(expr: Expr.Call): void {
    this.resolve_expr(expr.callee);

    for (const argument of expr.argv) {
      this.resolve_expr(argument);
    }
  }

  visit_get_expr(expr: Expr.Get): void {
    this.resolve_expr(expr.target);
  }

  visit_grouping_expr(expr: Expr.Grouping): void {
    this.resolve_expr(expr.expression);
  }

  visit_literal_expr(expr: Expr.Literal): void {
    void expr;
  }

  visit_logical_expr(expr: Expr.Logical): void {
    this.resolve_expr(expr.left);
    this.resolve_expr(expr.right);
  }

  visit_set_expr(expr: Expr.Set): void {
    this.resolve_expr(expr.value);
    this.resolve_expr(expr.target);
  }

  visit_super_expr(expr: Expr.Super): void {
    if (this.current_class === ClassKind.None) {
      Everest.error_with(expr.keyword, 'cannot use `super` outside of a class');
    } else if (this.current_class !== ClassKind.Subclass) {
      Everest.error_with(expr.keyword, '`super` has no target');
    }

    this.resolve_local(expr, expr.keyword);
  }

  visit_this_expr(expr: Expr.This): void {
    if (this.current_class === ClassKind.None) {
      Everest.error_with(expr.keyword, 'cannot `this` outside of a class.');
      return;
    }

    this.resolve_local(expr, expr.keyword);
  }

  visit_unary_expr(expr: Expr.Unary): void {
    this.resolve_expr(expr.right);
  }

  visit_variable_expr(expr: Expr.Variable): void {
    if (
      !this.scopes.is_empty() &&
      this.scopes.peek().get(expr.name.lexeme) === false
    ) {
      Everest.error_with(expr.name, 'cannot assign variable to itself');
    }

    this.resolve_local(expr, expr.name);
  }

  private resolve_stmt(stmt: Stmt): void {
    stmt.accept(this);
  }

  private resolve_expr(expr: Expr): void {
    expr.accept(this);
  }

  private resolve_fn(fn: Stmt.Fn, kind: FnKind): void {
    const enclosing_fn = this.current_fn;
    this.current_fn = kind;

    this.begin_scope();
    for (const param of fn.params) {
      this.declare(param);
      this.define(param);
    }
    this.end_scope();

    this.resolve(fn.body);

    this.current_fn = enclosing_fn;
  }

  private begin_scope(): void {
    this.scopes.push(new Map());
  }

  private end_scope(): void {
    this.scopes.pop();
  }

  private declare(name: Token): void {
    if (this.scopes.is_empty()) {
      return;
    }

    const scope = this.scopes.peek();
    if (scope.has(name.lexeme)) {
      Everest.error_with(name, 'variable already exists here');
    }

    scope.set(name.lexeme, false);
  }

  private define(name: Token): void {
    if (this.scopes.is_empty()) {
      return;
    }

    this.scopes.peek().set(name.lexeme, true);
  }

  private resolve_local(expr: Expr, name: Token): void {
    for (let i = this.scopes.size() - 1; i >= 0; i--) {
      if (this.scopes.get(i).has(name.lexeme)) {
        this.interpreter.resolve(expr, this.scopes.size() - 1 - i);
        return;
      }
    }
  }
}

export enum FnKind {
  None,
  Fn,
  Initializer,
  Method,
}

export enum ClassKind {
  None,
  Class,
  Subclass,
}
