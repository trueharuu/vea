import type { Expr } from './expr';
import type { Token } from './token';

export abstract class Stmt {
  abstract accept<R>(visitor: Stmt.Visitor<R>): R;
}

export namespace Stmt {
  export interface Visitor<R> {
    visit_block_stmt(stmt: Block): R;
    visit_class_stmt(stmt: Class): R;
    visit_expression_stmt(stmt: Expression): R;
    visit_fn_stmt(stmt: Fn): R;
    visit_if_stmt(stmt: If): R;
    visit_print_stmt(stmt: Print): R;
    visit_return_stmt(stmt: Return): R;
    visit_var_stmt(stmt: Var): R;
    visit_while_stmt(stmt: While): R;
    visit_for_stmt(stmt: For): R;
  }

  export class Block extends Stmt {
    constructor(public readonly statements: Array<Stmt>) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_block_stmt(this);
    }
  }

  export class Class extends Stmt {
    constructor(
      public readonly name: Token,
      public readonly superclass: Var,
      public readonly methods: Array<Fn>
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_class_stmt(this);
    }
  }

  export class Expression extends Stmt {
    constructor(public readonly expression: Expr) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_expression_stmt(this);
    }
  }

  export class Fn extends Stmt {
    constructor(
      public readonly name: Token,
      public readonly params: Array<Token>,
      public readonly body: Array<Stmt>
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_fn_stmt(this);
    }
  }

  export class If extends Stmt {
    constructor(
      public readonly condition: Expr,
      public readonly then_branch: Stmt,
      public readonly else_branch: Stmt | null
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_if_stmt(this);
    }
  }

  export class Print extends Stmt {
    constructor(public readonly expression: Expr) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_print_stmt(this);
    }
  }

  export class Return extends Stmt {
    constructor(
      public readonly keyword: Token,
      public readonly value: Expr | null
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_return_stmt(this);
    }
  }

  export class Var extends Stmt {
    constructor(
      public readonly name: Token,
      public readonly initializer: Expr | null
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_var_stmt(this);
    }
  }

  export class While extends Stmt {
    constructor(public readonly condition: Expr, public readonly body: Stmt) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_while_stmt(this);
    }
  }

  export class For extends Stmt {
    constructor(
      public readonly initializer: Stmt | null,
      public readonly condition: Expr | null,
      public readonly increment: Expr | null,
      public readonly body: Stmt
    ) {
      super();
    }

    accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_for_stmt(this);
    }
  }
}
