import type { Literal } from './token';

export interface Callable {
  arity(): number,
  call(interpreter: Interpreter, argv: Array<Literal>): Literal
}