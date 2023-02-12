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
## Eq: `x == y`
Returns `true` if `x` and `y` are equal. Implementing `Eq` implicitly creates this trait.
```rs
trait Eq<Y> {
   fn eq(&self, other: &Y) -> bool;
}
```
## Ne: `x != y`
Returns `true` if `x` and `y` are not equal. Implementing `Eq` implicitly creates this trait.
```rs
trait Ne<Y> {
   fn ne(&self, other: &Y) -> bool;
}
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
trait Cmp<Y> {
   fn cmp(&self, other: &Y) -> Ordering;
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
## Into: `x::T`
Converts `x` into type `T`
```rs
trait Into {
   type T;
   fn into(&self) -> Self::T;
}
```

## Index: `x[y]`
Gets the value for `y` in `x`.
```rs
trait Index<I> {
   type Output;
   fn index(&self, index: I) -> Self::Output;
}
```

## Assume: `x?`
Assumes `x` is a "correct" value. An example of this would be in `Result`.
```rs
enum T {
   Good(i32),
   Bad(i32),
}

Assume @ T {
   Output = i32;
   fn assume() {
      if self ? Good(m) {
         return m;
      } else {
         return !; // bad case will "never" happen
      }
   };
}

let value = T::Good(123);
assert(value? == 123);

let bad = T::Bad(123);
assert(value? == !);
```

```rs
trait Assume {
   type Output;
   fn assume(&self) -> Self::Output;
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
```rs
trait Copy {
   fn copy(&self) -> Self;
}
```

## Is: `x?y`
Returns `true` if `x` matches `y`.
```rs
trait Is<Y> {
   fn is(&self, other: &Y) -> bool;
}
```

# Types
* Integer: `123`, `5u8`
* Boolean: `true`, `false`
* String: `"abc"`
* Char: `'a'`
* None: `!`
* Environment: `env a`
* Set: `(1, 2, 3)`
* Array: `[1, 2, 2]`
* Function: `fn a() -> {}`

# Standard Library
[todo]()
