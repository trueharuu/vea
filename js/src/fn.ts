import { Callable } from './callable';
import { Environment } from './environment';
import type { Interpreter } from './interpreter';
import type { Return } from './return';
import type { Stmt } from './stmt';
import type { Literal } from './token';

export class Fn extends Callable {
  constructor(private readonly declaration: Stmt.Fn) {
    super();
  }

  public call(interpreter: Interpreter, argv: Array<Literal>): Literal {
    const env = new Environment(interpreter.globals);
    for (let i = 0; i < this.declaration.params.length; i++) {
      env.define(this.declaration.params[i]?.lexeme || '@', argv[i] || null);
    }

    try {
      interpreter.exec_block(this.declaration.body, env);
    } catch (e) {
      return (e as Return).value;
    }

    return null;
  }

  public arity(): number {
    return this.declaration.params.length;
  }
}
