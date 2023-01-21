export class Stack<T> {
  private readonly alloc: Array<T> = [];

  public peek(): T {
    const pop = this.alloc.pop();
    this.alloc.push(pop as T);
    return pop as T;
  }

  public is_empty(): boolean {
    return this.alloc.length === 0;
  }

  public push(value: T): void {
    this.alloc.push(value);
  }

  public pop(): T {
    return this.alloc.pop() as T;
  }

  public size(): number {
    return this.alloc.length;
  }

  public get(i: number): T {
    return this.alloc[i] as T;
  }
}
