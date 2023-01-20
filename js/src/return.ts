import type { Literal } from './token';

export class Return extends Error {
  public readonly value: Literal;
  constructor(value: Literal) {
    super(undefined);
    this.value = value;
  }
}
