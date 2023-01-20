import { RuntimeError } from './runtime_error';
import type { Literal, Token } from './token';

export class Environment {
  constructor(private readonly enclosing?: Environment) {}
  private readonly values: Map<string, Literal> = new Map();

  public get(name: Token): Literal {
    if (this.values.has(name.lexeme)) {
      return this.values.get(name.lexeme);
    }

    if (this.enclosing !== undefined) {
      return this.enclosing.get(name);
    }

    throw new RuntimeError(name, `undefined variable \`${name.lexeme}\``);
  }

  public assign(name: Token, value: Literal): void {
    if (this.values.has(name.lexeme)) {
      this.values.set(name.lexeme, value);
      return;
    }

    if (this.enclosing !== undefined) {
      this.enclosing.assign(name, value);
      return;
    }

    throw new RuntimeError(name, `undefined variable \`${name.lexeme}\``);
  }

  public define(name: string, value: Literal): void {
    this.values.set(name, value);
  }

  public ancestor(distance: number): Environment | undefined {
    // eslint-disable-next-line @typescript-eslint/no-this-alias
    let environment: Environment | undefined = this;
    for (let i = 0; i < distance; i++) {
      environment = environment?.enclosing;
    }

    return environment;
  }

  public get_at(distance: number, name: string): Literal {
    return this.ancestor(distance)?.values.get(name);
  }
}
