import { Callable } from './callable';
import type { Fn } from './fn';
import { Instance } from './instance';
import type { Interpreter } from './interpreter';
import type { Literal } from './token';

export class Class extends Callable {
  public readonly name: string;
  public readonly superclass?: Class;
  private readonly methods: Map<string, Fn>;
  constructor(
    name: string,
    superclass: Class | undefined,
    methods: Map<string, Fn>
  ) {
    super();
    this.superclass = superclass;
    this.name = name;
    this.methods = methods;
  }

  public find_method(name: string): Fn | undefined {
    if (this.methods.has(name)) {
      return this.methods.get(name);
    }

    if (this.superclass !== undefined) {
      return this.superclass.find_method(name);
    }

    return undefined;
  }

  // eslint-disable-next-line @typescript-eslint/naming-convention
  public toString(): string {
    return this.name;
  }

  public call(interpreter: Interpreter, argv: Array<Literal>): Literal {
    const instance = new Instance(this);
    const initializer = this.find_method('init');
    if (initializer !== undefined) {
      initializer.bind(instance).call(interpreter, argv);
    }

    return instance;
  }

  public arity(): number {
    const initializer = this.find_method('init');
    if (initializer === undefined) {
      return 0;
    }

    return initializer.arity();
  }
}
