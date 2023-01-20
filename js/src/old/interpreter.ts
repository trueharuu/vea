/* eslint-disable @typescript-eslint/no-unused-vars */
import { match } from '@rqft/rust';
import { Callable } from './callable';
import { Environment } from './environment';
import { Everest } from './everest';
import { Expr } from './expr';
import { Fn } from './fn';
import { Return } from './return';
import { RuntimeError } from './runtime_error';
import type { Stmt } from './stmt';
import type { Literal, Token } from './token';
import { TokenKind } from './token_kind';

export class Interpreter implements Expr.Visitor<Literal>, Stmt.Visitor<void> {
  readonly globals = new Environment();
  private environment = this.globals;

  constructor() {
    this.globals.define(
      'clock',
      new (class extends Callable {
        arity(): number {
          return 0;
        }

        call(): Literal {
          return Date.now() / 1000.0;
        }
      })()
    );
  }

  public interpret(statements: Array<Stmt>): void {
    try {
      for (const statement of statements) {
        this.exec(statement);
      }
    } catch (e) {
      Everest.runtime_error(e as RuntimeError);
    }
  }

  public stringify(obj: Literal): string {
    if (obj === null) {
      return 'none';
    }

    return obj.toString();
  }

  private eval(expr: Expr): Literal {
    return expr.accept(this);
  }

  private exec(stmt: Stmt): void {
    return stmt.accept(this);
  }

  private is_truthy(raw: Literal): boolean {
    return !(raw === null || raw === false);
  }

  visit_literal_expr(expr: Expr.Literal): Literal {
    return expr.value;
  }

  visit_grouping_expr(expr: Expr.Grouping): Literal {
    return this.eval(expr.expression);
  }

  visit_unary_expr(expr: Expr.Unary): Literal {
    const right = this.eval(expr.right);

    return match<TokenKind, Literal>(expr.operator.kind)
      .arm(TokenKind.Bang, () => !this.is_truthy(right))
      .arm(TokenKind.Minus, () => {
        this.check_number_operand(expr.operator, right);
        return -Number(right);
      })
      .output()
      .unwrap();
  }

  visit_binary_expr(expr: Expr.Binary): Literal {
    const left = this.eval(expr.left);
    const right = this.eval(expr.right);

    return match<TokenKind, Literal>(expr.operator.kind)
      .arm(TokenKind.Minus, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) - Number(right);
      })
      .arm(TokenKind.Plus, () => {
        if (typeof left === 'number' && typeof right === 'number') {
          return left + right;
        }

        if (typeof left === 'string' && typeof right === 'string') {
          return left + right;
        }

        throw new RuntimeError(
          expr.operator,
          'operands must be of type `string | number`'
        );
      })
      .arm(TokenKind.Slash, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) / Number(right);
      })
      .arm(TokenKind.Star, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) * Number(right);
      })
      .arm(TokenKind.Gt, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Ge, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) >= Number(right);
      })
      .arm(TokenKind.Lt, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) < Number(right);
      })
      .arm(TokenKind.Le, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) <= Number(right);
      })
      .arm(TokenKind.Ne, () => !this.is_eq(left, right))
      .arm(TokenKind.EqEq, () => this.is_eq(left, right))
      .output()
      .unwrap();
  }

  private is_eq(left: Literal, right: Literal): boolean {
    if (left === null && right === null) {
      return true;
    }
    if (left === null) {
      return false;
    }

    return left === right;
  }

  private check_number_operand(operator: Token, right: Literal): void {
    if (typeof right === 'number') {
      return;
    }

    throw new RuntimeError(operator, 'operand must be of type `number`');
  }

  private check_number_operands(
    operator: Token,
    left: Literal,
    right: Literal
  ): void {
    console.log(operator, left, right);
    if (typeof left === 'number' && typeof right === 'number') {
      return;
    }

    throw new RuntimeError(operator, 'operands must be of type `number`');
  }

  visit_assign_expr(expr: Expr.Assign): Literal {
    const value = this.eval(expr.value);
    this.environment.assign(expr.name, value);
    return value;
  }

  visit_call_expr(expr: Expr.Call): Literal {
    console.log('using call', expr);
    const callee = this.eval(expr.callee);

    const argv = [];
    for (const arg of expr.argv) {
      argv.push(this.eval(arg));
    }

    if (!(callee instanceof Callable)) {
      throw new RuntimeError(expr.paren, 'can only call fns and classes');
    }

    const fn = callee as Callable;

    if (argv.length !== fn.arity()) {
      throw new RuntimeError(
        expr.paren,
        `expected ${fn.arity()} arguments but recieved ${argv.length}`
      );
    }

    return fn.call(this, argv);
  }
  visit_get_expr(_expr: Expr.Get): Literal {
    throw new Error('Method not implemented.');
  }
  visit_logical_expr(expr: Expr.Logical): Literal {
    const left = this.eval(expr.left);
    if (expr.operator.kind === TokenKind.Or) {
      if (this.is_truthy(left)) {
        return left;
      }
    } else {
      if (!this.is_truthy(left)) {
        return left;
      }
    }

    return this.eval(expr.right);
  }
  visit_set_expr(_expr: Expr.Set): Literal {
    throw new Error('Method not implemented.');
  }
  visit_super_expr(_expr: Expr.Super): Literal {
    throw new Error('Method not implemented.');
  }
  visit_this_expr(_expr: Expr.This): Literal {
    throw new Error('Method not implemented.');
  }
  visit_variable_expr(expr: Expr.Variable): Literal {
    return this.environment.get(expr.name);
  }

  // statements

  visit_expression_stmt(stmt: Stmt.Expression): void {
    this.eval(stmt.expression);
  }

  visit_print_stmt(stmt: Stmt.Print): void {
    const value = this.eval(stmt.expression);
    console.log(this.stringify(value));
  }

  visit_block_stmt(stmt: Stmt.Block): void {
    this.exec_block(stmt.statements, new Environment(this.environment));
  }

  public exec_block(statements: Array<Stmt>, environment: Environment): void {
    const prev = this.environment;
    try {
      this.environment = environment;
      for (const statement of statements) {
        this.exec(statement);
      }
    } finally {
      this.environment = prev;
    }
  }

  visit_class_stmt(_stmt: Stmt.Class): void {
    throw new Error('Method not implemented.');
  }
  visit_fn_stmt(stmt: Stmt.Fn): void {
    const fn = new Fn(stmt);
    this.environment.define(stmt.name.lexeme, fn);
  }

  visit_if_stmt(stmt: Stmt.If): void {
    if (this.is_truthy(this.eval(stmt.condition))) {
      this.exec(stmt.then_branch);
    } else if (stmt.else_branch !== null) {
      this.exec(stmt.else_branch);
    }
  }
  visit_return_stmt(stmt: Stmt.Return): void {
    let value = null;
    if (stmt.value !== null) {
      value = this.eval(stmt.value);
    }

    throw new Return(value);
  }
  visit_var_stmt(stmt: Stmt.Var): void {
    let value = null;
    console.log('var', stmt);
    if (stmt.initializer !== null) {
      value = this.eval(stmt.initializer);
    }

    console.log('ok', value);

    this.environment.define(stmt.name.lexeme, value);

    console.log(stmt.name, this.environment.get(stmt.name));
  }
  visit_while_stmt(stmt: Stmt.While): void {
    while (this.is_truthy(this.eval(stmt.condition))) {
      this.exec(stmt.body);
    }
  }

  visit_for_stmt(stmt: Stmt.For): void {
    if (stmt.initializer) {
      this.exec(stmt.initializer);
    }

    while (this.eval(stmt.condition || new Expr.Literal(false))) {
      this.exec(stmt.body);
      if (stmt.increment) {
        this.eval(stmt.increment);
      }
    }
  }
}
