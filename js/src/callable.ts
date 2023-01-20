import type { Interpreter } from './interpreter';
import type { Literal } from './token';

export abstract class Callable {
  abstract arity(): number;
  abstract call(interpreter: Interpreter, argv: Array<Literal>): Literal;
  // eslint-disable-next-line @typescript-eslint/naming-convention
  abstract toString(): string;
}
