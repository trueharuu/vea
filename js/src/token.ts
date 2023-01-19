import type { TokenKind } from './token_kind';

export type Literal = boolean | number | string | null;

export class Token {
  public readonly kind: TokenKind;
  public readonly lexeme: string;
  public readonly literal: Literal;
  public readonly line: number;

  constructor(kind: TokenKind, lexeme: string, literal: Literal, line: number) {
    this.kind = kind;
    this.lexeme = lexeme;
    this.literal = literal;
    this.line = line;
  }

  // eslint-disable-next-line @typescript-eslint/naming-convention
  public toString(): string {
    return this.kind + ' ' + this.lexeme + ' ' + this.literal;
  }
}
