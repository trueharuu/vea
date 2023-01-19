import { RuntimeError } from './runtime_error';
import type { Literal, Token } from './token';

export class Environment {
  private readonly values = new Map<string, Literal>();
  constructor(private readonly enclosing: Environment | null = null) {}
  public define(name: string, value: Literal): void {
    this.values.set(name, value);
  }

  public get(name: Token): Literal {
    const raw = this.values.get(name.lexeme);

    if (raw !== undefined) {
      return raw;
    }

    if (this.enclosing !== null) {
      return this.enclosing.get(name);
    }

    throw new RuntimeError(name, `undefined variable \`${name.lexeme}\``);
  }

  public assign(name: Token, value: Literal): void {
    const raw = this.values.get(name.lexeme);
    if (raw !== undefined) {
      this.values.set(name.lexeme, value);
      return;
    }

    if (this.enclosing !== null) {
      this.enclosing.assign(name, value);
      return;
    }

    throw new RuntimeError(
      name,
      `cannot assign to undefined variable \`${name.lexeme}\`\n\tnote:use \`var\` to create a new variable instead: \`var ${name.lexeme} = ${value}\``
    );
  }
}
