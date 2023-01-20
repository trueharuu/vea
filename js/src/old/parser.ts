import { Everest } from './everest';
import { Expr } from './expr';
import { Stmt } from './stmt';
import type { Token } from './token';
import { TokenKind } from './token_kind';

export class Parser {
  private current = 0;
  constructor(private readonly tokens: Array<Token>) {}

  private expression(): Expr {
    return this.assignment();
  }

  private assignment(): Expr {
    const expr = this.or();
    if (this.is(TokenKind.Eq)) {
      const equals = this.prev();
      const value = this.assignment();

      if (expr instanceof Expr.Variable) {
        const name = expr.name;
        return new Expr.Assign(name, value);
      }

      this.error(equals, 'invalid assignment target');
    }

    return expr;
  }

  private or(): Expr {
    let expr = this.and();

    while (this.is(TokenKind.Or)) {
      const operator = this.prev();
      const right = this.and();
      expr = new Expr.Logical(expr, operator, right);
    }

    return expr;
  }

  private and(): Expr {
    let expr = this.equality();

    while (this.is(TokenKind.And)) {
      const operator = this.prev();
      const right = this.equality();
      expr = new Expr.Logical(expr, operator, right);
    }

    return expr;
  }

  public parse(): Array<Stmt | null> {
    const statements = [];
    while (!this.is_at_end()) {
      statements.push(this.declaration());
    }

    return statements;
  }

  private declaration(): Stmt | null {
    try {
      if (this.is(TokenKind.Fn)) {
        return this.fn('function');
      }

      if (this.is(TokenKind.Var)) {
        return this.var_declaration();
      }
      return this.statement();
    } catch {
      this.sync();
      return null;
    }
  }

  private fn(kind: string): Stmt.Fn {
    const name = this.consume(TokenKind.Identifier, `expected ${kind} name`);
    this.consume(TokenKind.LeftParen, `expected '(' after ${kind} name`);
    const params = [];

    if (!this.check(TokenKind.RightParen)) {
      do {
        if (params.length >= 255) {
          this.error(this.peek(), 'fn must have <= 255 parameters');
        }

        params.push(
          this.consume(TokenKind.Identifier, 'expected parameter name')
        );
      } while (this.is(TokenKind.Comma));
    }

    this.consume(TokenKind.RightParen, 'expected \')\' after parameters');

    this.consume(TokenKind.LeftBrace, `expected '{' before ${kind} body`);

    const body = this.block();
    return new Stmt.Fn(name, params, body);
  }

  private var_declaration(): Stmt {
    const name = this.consume(TokenKind.Identifier, 'expected variable name');

    let initializer = null;
    if (this.is(TokenKind.Eq)) {
      initializer = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after variable declaration');
    return new Stmt.Var(name, initializer);
  }

  private statement(): Stmt {
    if (this.is(TokenKind.For)) {
      return this.for_statement();
    }
    if (this.is(TokenKind.If)) {
      return this.if_statement();
    }
    if (this.is(TokenKind.Print)) {
      return this.print_statement();
    }
    if (this.is(TokenKind.Return)) {
      return this.return_statement();
    }
    if (this.is(TokenKind.While)) {
      return this.while_statement();
    }
    if (this.is(TokenKind.LeftBrace)) {
      return new Stmt.Block(this.block());
    }
    return this.expression_statement();
  }

  private return_statement(): Stmt {
    const keyword = this.prev();
    let value = null;
    if (!this.check(TokenKind.Semi)) {
      value = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after return value');

    return new Stmt.Return(keyword, value);
  }

  private for_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `for`');

    let initializer: Stmt | null;
    if (this.is(TokenKind.Semi)) {
      initializer = null;
    } else if (this.is(TokenKind.Var)) {
      initializer = this.var_declaration();
    } else {
      initializer = this.expression_statement();
    }

    let condition: Expr | null = null;
    if (!this.check(TokenKind.Semi)) {
      condition = this.expression();
    }

    this.consume(TokenKind.Semi, 'expected \';\' after loop condition');

    let incr: Expr | null = null;
    if (!this.check(TokenKind.RightParen)) {
      incr = this.expression();
    }

    this.consume(TokenKind.RightParen, 'expected \')\' after for clause');

    const body = this.statement();

    return new Stmt.For(initializer, condition, incr, body);
  }

  private while_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `while`');
    const condition = this.expression();
    this.consume(TokenKind.RightParen, 'expected \')\' after `while` condition');
    const body = this.statement();

    return new Stmt.While(condition, body);
  }

  private if_statement(): Stmt {
    this.consume(TokenKind.LeftParen, 'expected \'(\' after `if`');
    const condition = this.expression();
    this.consume(TokenKind.RightParen, 'expected \')\' after `if` condition');

    const then_branch = this.statement();
    let else_branch = null;
    if (this.is(TokenKind.Else)) {
      else_branch = this.statement();
    }

    return new Stmt.If(condition, then_branch, else_branch);
  }

  private block(): Array<Stmt> {
    const statements = [];

    while (!this.check(TokenKind.RightBrace) && !this.is_at_end()) {
      statements.push(this.declaration() as Stmt);
    }

    this.consume(TokenKind.RightBrace, 'expected \'}\' after block');
    return statements;
  }

  private print_statement(): Stmt {
    const value = this.expression();
    this.consume(TokenKind.Semi, 'expected \';\' after value');
    return new Stmt.Print(value);
  }

  private expression_statement(): Stmt {
    const expr = this.expression();
    this.consume(TokenKind.Semi, 'expected \';\' after expression');
    return new Stmt.Expression(expr);
  }

  public sync(): void {
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

  private equality(): Expr {
    let expr = this.comparison();
    while (this.is(TokenKind.Ne, TokenKind.EqEq)) {
      const operator = this.prev();
      const right = this.comparison();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private comparison(): Expr {
    let expr = this.term();
    while (this.is(TokenKind.Gt, TokenKind.Ge, TokenKind.Lt, TokenKind.Le)) {
      const operator = this.prev();
      const right = this.term();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private term(): Expr {
    let expr = this.factor();

    while (this.is(TokenKind.Minus, TokenKind.Plus)) {
      const operator = this.prev();
      const right = this.factor();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private factor(): Expr {
    let expr = this.unary();

    while (this.is(TokenKind.Slash, TokenKind.Star)) {
      const operator = this.prev();
      const right = this.unary();
      expr = new Expr.Binary(expr, operator, right);
    }

    return expr;
  }

  private unary(): Expr {
    if (this.is(TokenKind.Bang, TokenKind.Minus)) {
      const operator = this.prev();
      const right = this.unary();
      return new Expr.Unary(operator, right);
    }

    return this.call();
  }

  private call(): Expr {
    let expr = this.primary();
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition, no-constant-condition
    while (true) {
      if (this.is(TokenKind.LeftParen)) {
        expr = this.finish_call(expr);
      } else {
        break;
      }
    }

    return expr;
  }

  private finish_call(callee: Expr): Expr {
    const argv = [];
    if (!this.check(TokenKind.RightParen)) {
      do {
        if (argv.length >= 255) {
          this.error(this.peek(), 'fn must have <=255 arguments');
        }

        const a= this.expression();
        console.log('a', a);
        argv.push(a);
      } while (this.is(TokenKind.Comma));
    }

    const paren = this.consume(
      TokenKind.RightParen,
      'expected \')\' after fn arguments'
    );

    return new Expr.Call(callee, paren, argv);
  }

  private primary(): Expr {
    if (this.is(TokenKind.False)) {
      return new Expr.Literal(false);
    }

    if (this.is(TokenKind.True)) {
      return new Expr.Literal(true);
    }

    if (this.is(TokenKind.None)) {
      return new Expr.Literal(null);
    }

    if (this.is(TokenKind.Number, TokenKind.String)) {
      return new Expr.Literal(this.prev().literal);
    }

    if (this.is(TokenKind.Identifier)) {
      return new Expr.Variable(this.prev());
    }

    if (this.is(TokenKind.LeftParen)) {
      const expr = this.expression();
      this.consume(TokenKind.RightParen, 'expected \')\' after expression');
      return new Expr.Grouping(expr);
    }

    throw this.error(this.peek(), 'expected expression');
  }

  private consume(kind: TokenKind, message: string): Token {
    if (this.check(kind)) {
      return this.next();
    }
    throw this.error(this.peek(), message);
  }

  private error(token: Token, message: string): ParseError {
    Everest.error_with(token, message);

    return new ParseError();
  }

  private is(...kinds: Array<TokenKind>): boolean {
    for (const kind of kinds) {
      if (this.check(kind)) {
        this.next();
        return true;
      }
    }

    return false;
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
    return this.tokens[this.current] as never;
  }

  private prev(): Token {
    return this.tokens[this.current - 1] as never;
  }
}

export class ParseError extends Error {}
