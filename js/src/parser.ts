import { Everest } from './everest';
import { Expr } from './expr';
import { Stmt } from './stmt';
import type { Token } from './token';
import { TokenKind } from './token_kind';

export class Parser {
  private readonly tokens: Array<Token>;
  private current = 0;

  constructor(tokens: Array<Token>) {
    this.tokens = tokens;
  }

  public parse(): Array<Stmt | undefined> {
    const statements = [];
    while (!this.is_at_end()) {
      statements.push(this.declaration());
    }

    return statements;
  }

  private expression(): Expr {
    return this.assignment();
  }

  private declaration(): Stmt | undefined {
    try {
      if (this.match(TokenKind.Class)) {
        return this.class_declaration();
      }

      if (this.match(TokenKind.Fn)) {
        return this.fn('function');
      }

      if (this.match(TokenKind.Var)) {
        return this.var_declaration();
      }

      return this.statement();
    } catch (e) {
      this.sync();
      return undefined;
    }
  }

  private class_declaration(): Stmt {
    const name = this.consume(TokenKind.Identifier, 'expected class name');

    let super_class = undefined;
    if (this.match(TokenKind.Lt)) {
      this.consume(TokenKind.Identifier, 'expected superclass name');
      super_class = new Expr.Variable(this.prev());
    }

    this.consume(TokenKind.LeftBrace, 'expected \'{\' before class body');

    const methods = [];

    while (!this.check(TokenKind.RightBrace) && !this.is_at_end()) {
      methods.push(this.fn('method'));
    }

    this.consume(TokenKind.RightBrace, 'expected \'}\' after class body');

    return new Stmt.Class(name, super_class, methods);
  }

  private statement(): Stmt {
    if (this.match(TokenKind.For)) {
      return this.for_statement();
    }
    if (this.match(TokenKind.If)) {
      return this.if_statement();
    }
    if (this.match(TokenKind.Print)) {
      return this.print_statement();
    }
    if (this.match(TokenKind.Return)) {
      return this.return_statement();
    }
    if (this.match(TokenKind.While)) {
      return this.while_statement();
    }
    if (this.match(TokenKind.LeftBrace)) {
      return new Stmt.Block(this.block());
    }
    return this.expression_statement();
  }

  private for_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `for`');

    let initializer;
    if (this.match(TokenKind.Semi)) {
      initializer = undefined;
    } else if (this.match(TokenKind.Var)) {
      initializer = this.var_declaration();
    } else {
      initializer = this.expression_statement();
    }

    let condition = undefined;
    if (!this.check(TokenKind.Semi)) {
      condition = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after loop condition');

    let incr = undefined;
    if (!this.check(TokenKind.RightParen)) {
      incr = this.expression();
    }

    this.consume(TokenKind.RightParen, 'expected \')\' after for clauses');

    let body = this.statement();

    if (incr !== undefined) {
      body = new Stmt.Block([body, new Stmt.Expression(incr)]);
    }

    if (condition === undefined) {
      condition = new Expr.Literal(true);
    }

    body = new Stmt.While(condition, body);

    if (initializer !== undefined) {
      body = new Stmt.Block([initializer, body]);
    }

    return body;
  }

  private if_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `if`');
    const condition = this.expression();

    this.consume(TokenKind.RightParen, 'expected \')\' after `if` condition');

    const then_branch = this.statement();
    let else_branch = undefined;
    if (this.match(TokenKind.Else)) {
      else_branch = this.statement();
    }

    return new Stmt.If(condition, then_branch, else_branch);
  }

  private print_statement(): Stmt {
    const value = this.expression();
    this.consume(TokenKind.Semi, 'expected \';\' after value');
    return new Stmt.Print(value);
  }

  private return_statement(): Stmt {
    const keyword = this.prev();
    let value = undefined;
    if (!this.check(TokenKind.Semi)) {
      value = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after return value');
    return new Stmt.Return(keyword, value);
  }

  private var_declaration(): Stmt {
    const name = this.consume(TokenKind.Identifier, 'expected variable name');

    let initializer = undefined;
    if (this.match(TokenKind.Eq)) {
      initializer = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after variable declaration');
    return new Stmt.Var(name, initializer);
  }

  private while_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `while`');
    const condition = this.expression();
    this.consume(TokenKind.RightParen, 'expected \')\' after condition');
    const body = this.statement();

    return new Stmt.While(condition, body);
  }

  private expression_statement(): Stmt {
    const expr = this.expression();
    this.consume(TokenKind.Semi, 'expected \';\' after expression');
    return new Stmt.Expression(expr);
  }

  private fn(kind: string): Stmt.Fn {
    const name = this.consume(TokenKind.Identifier, `expected ${kind} name`);
    this.consume(TokenKind.LeftParen, `expected '(' after ${kind} name`);

    const parameters = [];
    if (!this.check(TokenKind.RightParen)) {
      do {
        if (parameters.length >= 255) {
          this.error(this.peek(), 'cannot have >255 parameters');
        }

        parameters.push(
          this.consume(TokenKind.Identifier, 'expected parameter name')
        );
      } while (this.match(TokenKind.Comma));
    }

    this.consume(TokenKind.RightParen, 'expected \')\' after parameters');
    this.consume(TokenKind.LeftBrace, `expected '{' before ${kind} body`);
    const body = this.block();
    return new Stmt.Fn(name, parameters, body);
  }

  private block(): Array<Stmt> {
    const statements = [];
    while (!this.check(TokenKind.RightBrace) && !this.is_at_end()) {
      statements.push(this.declaration() as Stmt);
    }

    this.consume(TokenKind.RightBrace, 'expected \'}\' after block');
    return statements;
  }

  private assignment(): Expr {
    const expr = this.or();

    if (this.match(TokenKind.Eq)) {
      const eq = this.prev();
      const value = this.assignment();
      if (expr instanceof Expr.Variable) {
        const name = expr.name;
        return new Expr.Assign(name, value);
      } else if (expr instanceof Expr.Get) {
        return new Expr.Set(expr.target, expr.name, value);
      }

      this.error(eq, 'invalid assignment target');
    }

    return expr;
  }

  private or(): Expr {
    let expr = this.and();

    while (this.match(TokenKind.Or)) {
      const operator = this.prev();
      const right = this.and();
      expr = new Expr.Logical(expr, operator, right);
    }

    return expr;
  }

  private and(): Expr {
    let expr = this.equality();

    while (this.match(TokenKind.And)) {
      const operator = this.prev();
      const right = this.equality();
      expr = new Expr.Logical(expr, operator, right);
    }

    return expr;
  }

  private equality(): Expr {
    let expr = this.comparison();
    while (this.match(TokenKind.Ne, TokenKind.EqEq)) {
      const operator = this.prev();
      const right = this.comparison();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private comparison(): Expr {
    let expr = this.term();
    while (this.match(TokenKind.Gt, TokenKind.Ge, TokenKind.Lt, TokenKind.Le)) {
      const operator = this.prev();
      const right = this.term();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private term(): Expr {
    let expr = this.factor();
    while (this.match(TokenKind.Minus, TokenKind.Plus)) {
      const operator = this.prev();
      const right = this.factor();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private factor(): Expr {
    let expr = this.unary();
    while (this.match(TokenKind.Slash, TokenKind.Star)) {
      const operator = this.prev();
      const right = this.unary();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private unary(): Expr {
    if (this.match(TokenKind.Bang, TokenKind.Minus)) {
      const operator = this.prev();
      const right = this.unary();
      return new Expr.Unary(operator, right);
    }

    return this.call();
  }

  private finish_call(callee: Expr): Expr {
    const argv = [];
    if (!this.check(TokenKind.RightParen)) {
      do {
        if (argv.length >= 255) {
          this.error(this.peek(), 'can\'t have more than 255 arguments');
        }

        argv.push(this.expression());
      } while (this.match(TokenKind.Comma));
    }

    const paren = this.consume(
      TokenKind.RightParen,
      'expected \')\' after arguments'
    );

    return new Expr.Call(callee, paren, argv);
  }

  private call(): Expr {
    let expr = this.primary();
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition, no-constant-condition
    while (true) {
      if (this.match(TokenKind.LeftParen)) {
        expr = this.finish_call(expr);
      } else if (this.match(TokenKind.Dot)) {
        const name = this.consume(
          TokenKind.Identifier,
          'expected property name'
        );
        expr = new Expr.Get(expr, name);
      } else {
        break;
      }
    }

    return expr;
  }

  private primary(): Expr {
    if (this.match(TokenKind.False)) { return new Expr.Literal(false); }
    if (this.match(TokenKind.True)) { return new Expr.Literal(true); }
    if (this.match(TokenKind.None)) { return new Expr.Literal(undefined); }
    
    if (this.match(TokenKind.Number, TokenKind.String)) {
      return new Expr.Literal(this.prev().literal);
    }

    if (this.match(TokenKind.Super)) {
      const keyword = this.prev();
      this.consume(TokenKind.Dot, 'expected \'.\' after super``');
      const method = this.consume(TokenKind.Identifier, 'expected super method name');
      return new Expr.Super(keyword, method);
    }

    if (this.match(TokenKind.This)) {
      return new Expr.This(this.prev());
    }

    if (this.match(TokenKind.Identifier)) {
      return new Expr.Variable(this.prev());
    }

    if (this.match(TokenKind.LeftParen)) {
      const expr = this.expression();
      this.consume(TokenKind.RightParen, 'expected \')\' after expression');
      return new Expr.Grouping(expr);
    }

    throw this.error(this.peek(), 'expected expression');
  }

  private match(...kinds: Array<TokenKind>): boolean {
    for (const kind of kinds) {
      if (this.check(kind)) {
        this.next();

        return true;
      }
    }

    return false;
  }

  private consume(kind: TokenKind, message: string): Token {
    if (this.check(kind)) {
      return this.next();
    }
    throw this.error(this.peek(), message);
  }

  private check(kind: TokenKind): boolean {
    if (this.is_at_end()) {
      return false;
    }
    return this.peek().kind === kind;
  }

  private next(): Token {
    if (!this.is_at_end()) {
      this.current++;
    }
    return this.prev();
  }

  private is_at_end(): boolean {
    return this.peek().kind === TokenKind.Eof;
  }

  private peek(): Token {
    return this.tokens[this.current] as Token;
  }

  private prev(): Token {
    return this.tokens[this.current - 1] as Token;
  }

  private error(token: Token, message: string): ParseError {
    Everest.error_with(token, message);
    return new ParseError();
  }

  private sync(): void {
    this.next();

    while (!this.is_at_end()) {
      if (this.prev().kind === TokenKind.Semi) {
        return;
      }

      switch (this.peek().kind) {
      case TokenKind.Class:
      case TokenKind.Fn:
      case TokenKind.Var:
      case TokenKind.For:
      case TokenKind.If:
      case TokenKind.While:
      case TokenKind.Print:
      case TokenKind.Return:
        return;
      }

      this.next();
    }
  }
}

export class ParseError extends Error {}
