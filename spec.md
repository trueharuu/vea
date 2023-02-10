# Operations

## Inverse: `!x`
Returns the inverse of an object. The expession `!!x == x` should always be true.
```rs
trait Inverse {
  type Output;
  fn inverse(&self) -> Self::Output;
}
```
## Not: `~x`
Returns the output of applying Logical NOT to the value.
This is the non-reflexive form of Inverse.
```rs
trait Not {
   type Output;
   fn not(&self) -> Self::Output;
}
```
## Or: `x | y`
Returns the output of applying Logical OR to the value.
```rs
trait Or<Y> {
   type Output;
   fn or(&self, other: &Y) -> Self::Output;
}
```
## And: `x & y`
Returns the output of applying Logical AND to the value.
```rs
trait And<Y> {
   type Output;
   fn and(&self, other: &Y) -> Self::Output;
}
```
## Xor: `x ^ y`
Returns the output of applying Logical XOR to the value.
```rs
trait Xor<Y> {
   type Output;
   fn xor(&self, other: &Y) -> Self::Output;
}
```
## Unwrap: `*x`
Returns the "inner value" of an object. By default, this is implemented for `env`s that hold a header value.
```rs
trait Unwrap {
   type Output;
   fn unwrap(&self) -> &Self::Output;
}
```
## Add: `x + y`
Assumes `(x + y) == (y + x)` by creating the `Add` implementation for `y`
```rs
trait Add<Y> {
   type Output;
   fn add(&self, other: &Y) -> Self::Output;
}
```
## Sub: `x - y`
```rs
trait Sub<Y> {
   type Output;
   fn sub(&self, other: &Y) -> Self::Output;
}
```
## Mul: `x * y`
Assumes `(x * y) == (y * x)` by creating the `Mul` implementation for `y`
```rs
trait Mul<Y> {
   type Output;
   fn mul(&self, other: &Y) -> Self::Output;
}
```
## Div: `x / y`
```rs
trait Div<Y> {
   type Output;
   fn div(&self, other: &Y) -> Self::Output;
}
```
Eq x == y
Ne x != y
Gt x >  y
Ge x >= y
Lt x <  y
Le x <= y
Assign x = y
Neg -x
IntoNumber +x
Into x::y
Index x[y]
Assume x?
Drop #x
Restrict x{y}
Copy &x

Types:
Integer    : Signed 64-bit integer: 123
Boolean    : true/false
String     : "abc"
Char       : 'a'
None       : !
Environment: env a;

Environments:
Act as both a variable and an environment. You can set properties of it and have treat it like a value.
```ts
env container = 1; // set the header value to 1.
container.a = 2; // set property 'a' to 2.
container = container.a + container; // set the header value to the value of the property 'a' (2) plus the header value (1)
```