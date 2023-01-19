import type { Expr } from './expr';

export class AstPrinter implements Expr.Visitor<string> {
  visit_assign_expr(expr: Expr.Assign): string {
    throw new Error('Method not implemented.');
  }
  visit_binary_expr(expr: Expr.Binary): string {
    throw new Error('Method not implemented.');
  }
  visit_call_expr(expr: Expr.Call): string {
    throw new Error('Method not implemented.');
  }
  visit_get_expr(expr: Expr.Get): string {
    throw new Error('Method not implemented.');
  }
  visit_grouping_expr(expr: Expr.Grouping): string {
    throw new Error('Method not implemented.');
  }
  visit_literal_expr(expr: Expr.Literal): string {
    throw new Error('Method not implemented.');
  }
  visit_logical_expr(expr: Expr.Logical): string {
    throw new Error('Method not implemented.');
  }
  visit_set_expr(expr: Expr.Set): string {
    throw new Error('Method not implemented.');
  }
  visit_super_expr(expr: Expr.Super): string {
    throw new Error('Method not implemented.');
  }
  visit_this_expr(expr: Expr.This): string {
    throw new Error('Method not implemented.');
  }
  visit_unary_expr(expr: Expr.Unary): string {
    throw new Error('Method not implemented.');
  }
  visit_variable_expr(expr: Expr.Variable): string {
    throw new Error('Method not implemented.');
  }
}
