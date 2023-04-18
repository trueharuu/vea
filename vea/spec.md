# Operations

## Inverse: `!x`
Returns the inverse of an object. The expession `!!x == x` should always be true.
```rs
trait Inverse {
  type Output;
  fn inverse(&self) -> Self::Output;
}
```
```rs
a = 1u8;
assert_eq!(!a, 254u8);
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
```rs
a = 1u8;
assert_eq!(~a, 254u8);
```

## Or: `x | y`
Returns the output of applying Logical OR to the value.
```rs
trait Or<Y> {
   type Output;
   fn or(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(5 | 3, 7);
```
## And: `x & y`
Returns the output of applying Logical AND to the value.
```rs
trait And<Y> {
   type Output;
   fn and(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(5 & 3, 1);
```
## Xor: `x ^ y`
Returns the output of applying Logical XOR to the value.
```rs
trait Xor<Y> {
   type Output;
   fn xor(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(5 ^ 3, 6);
```
## Unwrap: `*x`
Returns the "inner value" of an object. By default, this is implemented for `env`s that hold a header value.
```rs
trait Unwrap {
   type Output;
   fn unwrap(&self) -> &Self::Output;
}
```
```rs
// here is how to make a class !
fn struct(value) -> {
   env body;
   body.value = value;
   Unwrap @ body {
      value
   }

   return body;
};

assert_eq!(*struct(1), 1);
```
## Add: `x + y`
Assumes `(x + y) == (y + x)` by creating the `Add` implementation for `y`
```rs
trait Add<Y> {
   type Output;
   fn add(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(1 + 1, 2);
```
## Sub: `x - y`
```rs
trait Sub<Y> {
   type Output;
   fn sub(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(1 - 1, 0);
```
## Mul: `x * y`
Assumes `(x * y) == (y * x)` by creating the `Mul` implementation for `y`
```rs
trait Mul<Y> {
   type Output;
   fn mul(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(9 * 5, 45);
```
## Div: `x / y`
```rs
trait Div<Y> {
   type Output;
   fn div(&self, other: &Y) -> Self::Output;
}
```
```rs
assert_eq!(1 / 2, frac(1, 2));
```
## Eq: `x == y`
Returns `true` if `x` and `y` are equal. Implementing `Eq` implicitly creates this trait.
```rs
trait Eq<Y> {
   fn eq(&self, other: &Y) -> bool;
}
```
```rs
assert_eq!(1 == 1, true);
```
## Ne: `x != y`
Returns `true` if `x` and `y` are not equal. Implementing `Eq` implicitly creates this trait.
```rs
trait Ne<Y> {
   fn ne(&self, other: &Y) -> bool;
}
```
```rs
assert_eq!(1 != 1, false);
```

## Gt: `x > y`
Returns `true` if `x` is explicitly greater than `y`. Implementing this implicitly creates `Le`, `Lt`, and `Ge`.
```rs
trait Gt<Y> {
   fn gt(&self, other: &Y) -> bool;
}
```
## Ge: `x >= y`
Returns `true` if `x` is greater than or equal to `y`. Implementing this implicitly creates `Le`, `Lt`, and `Gt`.
```rs
trait Ge<Y> {
   fn ge(&self, other: &Y) -> bool;
}
```
## Lt: `x < y`
Returns `true` if `x` is explicitly less than `y`. Implementing this implicitly creates `Le`, `Gt`, and `Ge`.
```rs
trait Lt<Y> {
   fn lt(&self, other: &Y) -> bool;
}
```
## Le: `x <= y`
Returns `true` if `x` is less than or equal to `y`. Implementing this implicitly creates `Gt`, `Lt`, and `Ge`.
```rs
trait Le<Y> {
   fn le(&self, other: &Y) -> bool;
}
```

## Cmp
Covers all of the `Gt`, `Ge`, `Lt`, `Le` traits.
```rs
enum Ordering {
   GreaterThan, GreaterThanOrEqual,
   LessThan, LessThanOrEqual,
   Equal, NotEqual
}

trait Cmp<Y> {
   fn cmp(&self, other: &Y) -> Ordering;
}
```
```rs
assert_eq!(1 > 2, false);
```

## Epsilon
`Cmp`, with a range of "accepted values". `1 >~1 2` means "one is greater than the entire range of 1, for 2"
```rs
enum TotalOrdering {
   GreaterThan, // >
   GreaterThanOrEqual, // >=
   LessThan, // <
   LessThanOrEqual, // <=
   Equal, // ==
   NotEqual, // !=
   AlmostGreaterThan, // >~n
   AlmostGreaterThanOrEqual, // >=~n
   AlmostLessThan, // <~n
   AlmostLessThanOrEqual, // <=~n
   AlmostEqual, // ==~n
}
trait Epsilon<Y> {
   fn cmp(&self, other: &Y) where Self: Cmp -> TotalOrdering;
}
```

## Neg: `-x`
Returns the negative of `x`. The operation `--x` should return `x`.
```rs
trait Neg {
   type Output;
   fn neg(&self) -> Self::Output;
}
```
```rs
a = 1;
b = -1;
assert_eq!(-a, b);
assert_eq!(a, --a);
```
## Into: `x::T`
Converts `x` into type `T`
```rs
trait Into {
   type T;
   fn into(&self) -> Self::T;
}
```
```rs
a = (1, 2, 3);
b = [1, 2, 3];
assert_eq!(a::List, b);
```

## Index: `x[y]`
Gets the value for `y` in `x`. Please note that the built-in `Set`s are unordered, and cannot be indexed into.
```rs
trait Index<I> {
   type Output;
   fn index(&self, index: I) -> Self::Output;
}
```
```rs
a = [1, 2, 3];
assert_eq!(a[1], 2);
```

## Assume: `x?`
Assumes `x` is a "correct" value. An example of this would be in `Result`.
```rs
trait Assume {
   type Output;
   fn assume(&self) -> Self::Output;
}
```
```rs
fn T(marker, value) -> {
   env body;
   // `move` drops `marker`, moving it into body;
   body.marker = #marker;

}
```

## Drop: `#x`
Destroys a value.
```rs
trait Drop {
   fn drop(&self) -> !;
}
```

## Restrict: `x{y}`
Converts `x` to `!` if the output of `y` is `false`.
```rs
trait Restrict {
   fn restrict(&self, expr: Fn) -> Self;
}
```

## Copy: `&x`
Copies a value. This is automatically for implemented for everything, and can be removed if the output is `!`.
This removes the `lock` state for the copy if there is one.
```rs
trait Copy {
   fn copy(&self) -> Self;
}
```

## Lock: `lock x`
Marks a value as immutable. You can only `Drop` or `Copy` the value afterwards. In the case of an `env`, you can still change unlocked properties.

## Is: `x?y`
Returns `true` if `x` matches `y`.
```rs
trait Is<Y> {
   fn is(&self, other: &Y) -> bool;
}
```

# Types
* Integer: `123`, `5u8`
  * Any Integer type that can be represented in Rust.
* Boolean: `true`, `false`
* String: `"abc"`
* Char: `'a'`
* None: `!`
* Environment: `env a`
* Set: `(1, 2, 3)`
  * Items must be homogenous and unique.
* Array: `[1, 2, 2]`
* Function: `fn a() -> {}`

# Standard Library
[todo]()
