import { match } from '@rqft/rust';
import { performance } from 'perf_hooks';
import { Callable } from './callable';
import { Class } from './class';
import { Environment } from './environment';
import { Everest } from './everest';
import type { Expr } from './expr';
import { Fn } from './fn';
import { Instance } from './instance';
import { Return } from './return';
import { RuntimeError } from './runtime_error';
import type { Stmt } from './stmt';
import type { Literal, Token } from './token';
import { TokenKind } from './token_kind';

export class Interpreter implements Expr.Visitor<Literal>, Stmt.Visitor<void> {
  public readonly globals = new Environment();
  private environment = this.globals;
  private readonly locals = new Map<Expr, number>();

  constructor() {
    this.globals.define(
      'clock',
      new (class extends Callable {
        public arity(): number {
          return 0;
        }

        public call(): Literal {
          return performance.now() / 1000.0;
        }

        // eslint-disable-next-line @typescript-eslint/naming-convention
        public toString(): string {
          return '<native fn>';
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
      return Everest.runtime_error(e as RuntimeError);
    }
  }

  private eval(expr: Expr): Literal {
    return expr.accept(this);
  }

  private exec(stmt: Stmt): void {
    stmt.accept(this);
  }

  public resolve(expr: Expr, depth: number): void {
    this.locals.set(expr, depth);
  }

  public exec_block(statements: Array<Stmt>, environment: Environment): void {
    const previous = this.environment;
    try {
      this.environment = environment;
      for (const statement of statements) {
        this.exec(statement);
      }
    } finally {
      this.environment = previous;
    }
  }

  public visit_block_stmt(stmt: Stmt.Block): void {
    this.exec_block(stmt.statements, new Environment(this.environment));
  }

  public visit_class_stmt(stmt: Stmt.Class): void {
    let superclass: Class | undefined = undefined;
    if (stmt.superclass !== undefined) {
      superclass = this.eval(stmt.superclass) as Class;
      if (!((superclass as unknown) instanceof Class)) {
        throw new RuntimeError(stmt.superclass.name, 'super must be a class');
      }
    }

    this.environment.define(stmt.name.lexeme, undefined);

    if (stmt.superclass !== undefined) {
      this.environment = new Environment(this.environment);
      this.environment.define('super', stmt.superclass);
    }

    const methods = new Map<string, Fn>();
    for (const method of stmt.methods) {
      const fn = new Fn(
        method,
        this.environment,
        method.name.lexeme === 'init'
      );
      methods.set(method.name.lexeme, fn);
    }

    const klass = new Class(stmt.name.lexeme, superclass as Class, methods);

    if (superclass !== undefined) {
      this.environment = this.environment.enclosing as Environment;
    }

    this.environment.assign(stmt.name, klass);
  }

  visit_expression_stmt(stmt: Stmt.Expression): void {
    this.eval(stmt.expression);
  }

  visit_fn_stmt(stmt: Stmt.Fn): void {
    const fn = new Fn(stmt, this.environment, false);
    this.environment.define(stmt.name.lexeme, fn);
  }

  visit_if_stmt(stmt: Stmt.If): void {
    if (this.is_truthy(this.eval(stmt.condition))) {
      this.exec(stmt.then_branch);
    } else if (stmt.else_branch !== undefined) {
      this.exec(stmt.else_branch);
    }
  }

  visit_print_stmt(stmt: Stmt.Print): void {
    const value = this.eval(stmt.expression);
    console.log(this.stringify(value));
  }

  visit_return_stmt(stmt: Stmt.Return): void {
    let value = undefined;
    if (stmt.value !== undefined) {
      value = this.eval(stmt.value);
    }
    throw new Return(value);
  }

  visit_var_stmt(stmt: Stmt.Var): void {
    let value = undefined;
    if (stmt.initializer !== undefined) {
      value = this.eval(stmt.initializer);
    }

    this.environment.define(stmt.name.lexeme, value);
  }

  visit_while_stmt(stmt: Stmt.While): void {
    while (this.is_truthy(this.eval(stmt.condition))) {
      this.exec(stmt.body);
    }
  }

  visit_assign_expr(expr: Expr.Assign): Literal {
    const value = this.eval(expr.value);
    const distance = this.locals.get(expr);
    if (distance !== undefined) {
      this.environment.assign_at(distance, expr.name, value);
    } else {
      this.globals.assign(expr.name, value);
    }

    return value;
  }

  visit_binary_expr(expr: Expr.Binary): Literal {
    const left = this.eval(expr.left);
    const right = this.eval(expr.right);

    return match<TokenKind, Literal>(expr.operator.kind)
      .arm(TokenKind.Ne, !this.is_eq(left, right))
      .arm(TokenKind.EqEq, this.is_eq(left, right))
      .arm(TokenKind.Gt, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Ge, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Lt, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Le, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Minus, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
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
        return Number(left) > Number(right);
      })
      .arm(TokenKind.Star, () => {
        this.check_number_operands(expr.operator, left, right);
        return Number(left) > Number(right);
      })
      .output()
      .unwrap();
  }

  visit_call_expr(expr: Expr.Call): Literal {
    const callee = this.eval(expr.callee);
    const argv = [];
    for (const argument of expr.argv) {
      argv.push(this.eval(argument));
    }

    if (!(callee instanceof Callable)) {
      throw new RuntimeError(expr.paren, 'can only call fns and classes');
    }

    const fn = callee;
    if (argv.length !== fn.arity()) {
      throw new RuntimeError(
        expr.paren,
        `expected ${fn.arity()} arguments but got ${argv.length}`
      );
    }

    return fn.call(this, argv);
  }

  visit_get_expr(expr: Expr.Get): Literal {
    const target = this.eval(expr.target);
    if (target instanceof Instance) {
      return target.get(expr.name);
    }

    throw new RuntimeError(expr.name, 'cannot get properties of primitive');
  }

  visit_grouping_expr(expr: Expr.Grouping): Literal {
    return this.eval(expr.expression);
  }

  visit_literal_expr(expr: Expr.Literal): Literal {
    return expr.value;
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

  visit_set_expr(expr: Expr.Set): Literal {
    const target = this.eval(expr.target);

    if (!(target instanceof Instance)) {
      throw new RuntimeError(expr.name, 'cannot set properties of primitive');
    }

    const value = this.eval(expr.value);
    target.set(expr.name, value);
    return value;
  }

  visit_super_expr(expr: Expr.Super): Literal {
    const distance = this.locals.get(expr) || 0;

    const superclass = this.environment.get_at(distance, 'super') as Class;

    const target = this.environment.get_at(distance - 1, 'this') as Instance;

    const method = superclass.find_method(expr.method.lexeme);

    if (method === undefined) {
      throw new RuntimeError(
        expr.method,
        `undefined property \`${expr.method.lexeme}\``
      );
    }

    return method.bind(target);
  }

  visit_this_expr(expr: Expr.This): Literal {
    return this.lookup_variable(expr.keyword, expr);
  }

  visit_unary_expr(expr: Expr.Unary): Literal {
    const right = this.eval(expr.right);

    return match<TokenKind, Literal>(expr.operator.kind)
      .arm(TokenKind.Bang, !this.is_truthy(right))
      .arm(TokenKind.Minus, () => {
        this.check_number_operand(expr.operator, right);
        return -(right as number);
      })
      .output()
      .unwrap();
  }

  visit_variable_expr(expr: Expr.Variable): Literal {
    return this.lookup_variable(expr.name, expr);
  }

  private lookup_variable(name: Token, expr: Expr): Literal {
    const distance = this.locals.get(expr);
    if (distance !== undefined) {
      return this.environment.get_at(distance, name.lexeme);
    } else {
      return this.globals.get(name);
    }
  }

  private check_number_operand(operator: Token, operand: Literal): void {
    if (typeof operand !== 'number') {
      throw new RuntimeError(operator, 'operand must be of type `number`');
    }
  }

  private check_number_operands(
    operator: Token,
    left: Literal,
    right: Literal
  ): void {
    if (!(typeof left === 'number' && typeof right === 'number')) {
      throw new RuntimeError(operator, 'operands must be of type `number`');
    }
  }

  private is_truthy(object: Literal): boolean {
    if (object === undefined || object === false) {
      return false;
    }
    return true;
  }

  private is_eq(a: Literal, b: Literal): boolean {
    // if (a === undefined && b === undefined) { return true; }
    // if (a === undefined) { return false; }
    if (a === undefined) {
      return b === undefined;
    }
    return a === b;
  }

  private stringify(obj: Literal): string {
    if (obj === undefined) {
      return 'none';
    }
    return obj?.toString() || 'none';
  }
}
