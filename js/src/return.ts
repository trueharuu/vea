import type { Literal } from './token';

export class Return extends Error {
  constructor(public readonly value: Literal) {
    super(undefined);
    this.stack = undefined;
  }
}
