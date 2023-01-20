import type { Interpreter } from './interpreter';
import type { Literal } from './token';

export abstract class Callable {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  abstract call(interpreter: Interpreter, argv: Array<Literal>): any;
  abstract arity(): number
}
