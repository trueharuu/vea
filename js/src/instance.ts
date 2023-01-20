import type { Class } from './class';
import { RuntimeError } from './runtime_error';
import type { Literal, Token } from './token';

export class Instance {
  private klass: Class;
  private readonly fields = new Map();

  constructor(klass: Class) {
    this.klass = klass;
  }

  public get(name: Token): Literal {
    if (this.fields.has(name.lexeme)) {
      return this.fields.get(name.lexeme);
    }

    const method = this.klass.find_method(name.lexeme);
    if (method !== undefined) {
      return method.bind(this);
    }

    throw new RuntimeError(name, `undefined property \`${name.lexeme}\``);
  }

  public set(name: Token, value: Literal): void {
    this.fields.set(name.lexeme, value);
  }

  // eslint-disable-next-line @typescript-eslint/naming-convention
  public toString(): string {
    return `${this.klass.name} instance`;
  }
}
