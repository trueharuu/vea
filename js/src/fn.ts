import { Callable } from './callable';
import { Environment } from './environment';
import type { Instance } from './instance';
import type { Interpreter } from './interpreter';
import { Return } from './return';
import type { Stmt } from './stmt';
import type { Literal } from './token';

export class Fn extends Callable {
  private readonly declaration: Stmt.Fn;
  private readonly closure: Environment;
  private readonly is_initializer: boolean;

  constructor(
    declaration: Stmt.Fn,
    closure: Environment,
    is_initializer: boolean
  ) {
    super();
    this.is_initializer = is_initializer;
    this.closure = closure;
    this.declaration = declaration;
  }

  public bind(instance: Instance): Fn {
    const environment = new Environment(this.closure);
    environment.define('this', instance);
    return new Fn(this.declaration, environment, this.is_initializer);
  }

  // eslint-disable-next-line @typescript-eslint/naming-convention
  public toString(): string {
    return `<fn ${this.declaration.name.lexeme}>`;
  }

  public arity(): number {
    return this.declaration.params.length;
  }

  public call(interpreter: Interpreter, argv: Array<Literal>): Literal {
    const environment = new Environment(this.closure);
    for (let i = 0; i < this.declaration.params.length; i++) {
      environment.define(this.declaration.params[i]?.lexeme as string, argv[i]);
    }

    try {
      interpreter.exec_block(this.declaration.body, environment);
    } catch (e) {
      if (this.is_initializer) {
        return this.closure.get_at(0, 'this');
      }

      if (e instanceof Return) {
        return e.value;
      }

      throw e;
    }

    if (this.is_initializer) {
      return this.closure.get_at(0, 'this');
    }
  }
}
