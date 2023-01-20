import { match } from '@rqft/rust';
import { Everest } from './everest';
import type { Literal } from './token';
import { Token } from './token';
import { TokenKind } from './token_kind';
export class Scanner {
  private readonly source: string;
  private readonly tokens: Array<Token> = [];
  private start = 0;
  private current = 0;
  private line = 1;

  constructor(source: string) {
    this.source = source;
  }

  public scan_tokens(): Array<Token> {
    while (!this.is_at_end()) {
      this.start = this.current;
      this.scan_token();
    }

    this.tokens.push(new Token(TokenKind.Eof, '', null, this.line));
    return this.tokens;
  }

  private is_at_end(): boolean {
    return this.current >= this.source.length;
  }

  private scan_token(): void {
    const c: string = this.next();
    match<string, void>(c)
      .arm('(', () => this.add_token(TokenKind.LeftParen))
      .arm(')', () => this.add_token(TokenKind.RightParen))
      .arm('{', () => this.add_token(TokenKind.LeftBrace))
      .arm('}', () => this.add_token(TokenKind.RightBrace))
      .arm(',', () => this.add_token(TokenKind.Comma))
      .arm('.', () => this.add_token(TokenKind.Dot))
      .arm('-', () => this.add_token(TokenKind.Minus))
      .arm('+', () => this.add_token(TokenKind.Plus))
      .arm(';', () => this.add_token(TokenKind.Semi))
      .arm('*', () => this.add_token(TokenKind.Star))
      .arm('!', () =>
        this.add_token(this.is('=') ? TokenKind.Ne : TokenKind.Bang)
      )
      .arm('=', () =>
        this.add_token(this.is('=') ? TokenKind.EqEq : TokenKind.Eq)
      )
      .arm('<', () =>
        this.add_token(this.is('=') ? TokenKind.Le : TokenKind.Lt)
      )
      .arm('>', () =>
        this.add_token(this.is('=') ? TokenKind.Ge : TokenKind.Gt)
      )
      .arm('/', () => {
        if (this.is('/')) {
          while (this.peek() !== '\n' && !this.is_at_end()) {
            this.next();
          }
        } else {
          this.add_token(TokenKind.Slash);
        }
      })
      .union([' ', '\r', '\t'], undefined)
      .arm('\n', () => this.line++)
      .arm('"', () => this.string())

      .wildcard((c) => {
        if (this.is_digit(c)) {
          this.number();
        } else if (this.is_alpha(c)) {
          this.identifier();
        } else {
          Everest.error(this.line, 'unexpected char');
        }
      })
      .output();
  }

  private identifier(): void {
    while (this.is_alphanumeric(this.peek())) {
      this.next();
    }

    const text = this.source.substring(this.start, this.current);
    const type = this.keywords().get(text) || TokenKind.Identifier;

    this.add_token(type);
  }

  private keywords(): Map<string, TokenKind> {
    const map = new Map<string, TokenKind>();

    map.set('and', TokenKind.And);
    map.set('class', TokenKind.Class);
    map.set('else', TokenKind.Else);
    map.set('false', TokenKind.False);
    map.set('for', TokenKind.For);
    map.set('fn', TokenKind.Fn);
    map.set('if', TokenKind.If);
    map.set('none', TokenKind.None);
    map.set('or', TokenKind.Or);
    map.set('print', TokenKind.Print);
    map.set('return', TokenKind.Return);
    map.set('super', TokenKind.Super);
    map.set('this', TokenKind.This);
    map.set('true', TokenKind.True);
    map.set('var', TokenKind.Var);
    map.set('while', TokenKind.While);

    return map;
  }

  private is_alpha(c: string): boolean {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
  }

  private is_alphanumeric(c: string): boolean {
    return this.is_digit(c) || this.is_alpha(c);
  }

  private is_digit(char: string): boolean {
    return char >= '0' && char <= '9';
  }

  private number(): void {
    while (this.is_digit(this.peek())) {
      this.next();
    }

    if (this.peek() === '.' && this.is_digit(this.peek_next())) {
      this.next();

      while (this.is_digit(this.peek())) {
        this.next();
      }
    }

    this.add_token_with(
      TokenKind.Number,
      Number(this.source.substring(this.start, this.current))
    );
  }

  private peek_next(): string {
    if (this.current + 1 >= this.source.length) {
      return '\0';
    }

    return this.source.charAt(this.current + 1);
  }

  private string(): void {
    while (this.peek() !== '"' && !this.is_at_end()) {
      if (this.peek() === '\n') {
        this.line++;
      }
      this.next();
    }

    if (this.is_at_end()) {
      Everest.error(this.line, 'unterminated string');
      return;
    }

    this.next();

    const value = this.source.substring(this.start + 1, this.current - 1);
    this.add_token_with(TokenKind.String, value);
  }

  private next(): string {
    return this.source.charAt(this.current++);
  }

  private add_token(kind: TokenKind): void {
    this.add_token_with(kind, null);
  }

  private add_token_with(kind: TokenKind, literal: Literal): void {
    const text = this.source.substring(this.start, this.current);
    this.tokens.push(new Token(kind, text, literal, this.line));
  }

  private is(expected: string): boolean {
    if (this.is_at_end()) {
      return false;
    }

    if (this.source.charAt(this.current) !== expected) {
      return false;
    }

    this.current++;
    return true;
  }

  private peek(): string {
    if (this.is_at_end()) {
      return '\0';
    }

    return this.source.charAt(this.current);
  }
}
