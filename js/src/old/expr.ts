
import type { Literal as RawLiteral, Token } from './token';

export abstract class Expr {
  public abstract accept<R>(visitor: Expr.Visitor<R>): R;
}

export namespace Expr {
  export interface Visitor<R> {
    visit_assign_expr(expr: Assign): R;
    visit_binary_expr(expr: Binary): R;
    visit_call_expr(expr: Call): R;
    visit_get_expr(expr: Get): R;
    visit_grouping_expr(expr: Grouping): R;
    visit_literal_expr(expr: Literal): R;
    visit_logical_expr(expr: Logical): R;
    visit_set_expr(expr: Set): R;
    visit_super_expr(expr: Super): R;
    visit_this_expr(expr: This): R;
    visit_unary_expr(expr: Unary): R;
    visit_variable_expr(expr: Variable): R;
  }

  export class Assign extends Expr {
    constructor(public readonly name: Token, public readonly value: Expr) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_assign_expr(this);
    }
  }

  export class Binary extends Expr {
    constructor(
      public readonly left: Expr,
      public readonly operator: Token,
      public readonly right: Expr
    ) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_binary_expr(this);
    }
  }

  export class Call extends Expr {
    constructor(
      public readonly callee: Expr,
      public readonly paren: Token,
      public readonly argv: Array<Expr>
    ) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_call_expr(this);
    }
  }

  export class Get extends Expr {
    constructor(public readonly target: Expr, public readonly name: Token) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_get_expr(this);
    }
  }

  export class Grouping extends Expr {
    constructor(public readonly expression: Expr) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_grouping_expr(this);
    }
  }

  export class Literal extends Expr {
    constructor(public readonly value: RawLiteral) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_literal_expr(this);
    }
  }

  export class Logical extends Expr {
    constructor(
      public readonly left: Expr,
      public readonly operator: Token,
      public readonly right: Expr
    ) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_logical_expr(this);
    }
  }

  export class Set extends Expr {
    constructor(
      public readonly target: Expr,
      public readonly name: Token,
      public readonly value: Expr
    ) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_set_expr(this);
    }
  }

  export class Super extends Expr {
    constructor(public readonly keyword: Token, public readonly method: Token) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_super_expr(this);
    }
  }

  export class This extends Expr {
    constructor(public readonly keyword: Token) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_this_expr(this);
    }
  }

  export class Unary extends Expr {
    constructor(public readonly operator: Token, public readonly right: Expr) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_unary_expr(this);
    }
  }

  export class Variable extends Expr {
    constructor(public readonly name: Token) {
      super();
    }

    public accept<R>(visitor: Visitor<R>): R {
      return visitor.visit_variable_expr(this);
    }
  }
}
