`pub fn `[`try_map`](https://doc.rust-lang.org/nightly/std/primitive.array.html#method.try_map)`<F, R>(`<br>
`  self,`<br>
`  f: F`<br>
`) -> <<R as `[`Try`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html)`>::`[`Residual`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html#associatedtype.Residual)`as`[`Residual`](https://doc.rust-lang.org/nightly/std/ops/trait.Residual.html)`<[<R as `[`Try`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html)`>::`[`Output`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html#associatedtype.Output)`; `[`N`](https://doc.rust-lang.org/nightly/std/primitive.array.html)`]>>::`[`TryType`](https://doc.rust-lang.org/nightly/std/ops/trait.Residual.html#associatedtype.TryType)<br>
`where`<br>
` F:`[`FnMut`](https://doc.rust-lang.org/nightly/std/ops/trait.Residual.html#associatedtype.TryType)`(T) -> R`<br>
` R:`[`Try`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html)<br>
` <R as`[`Try`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html)`>::`[`Residual`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html#associatedtype.Residual)`: `[`Residual`](https://doc.rust-lang.org/nightly/std/ops/trait.Residual.html)`<[<R as `[`Try`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html)`>::`[`Output`](https://doc.rust-lang.org/nightly/std/ops/trait.Try.html#associatedtype.Output)`; `[`N`](https://doc.rust-lang.org/nightly/std/primitive.array.html)`]>`

> This is a nightly-only experimental API. (`array_try_map` [#79711](https://github.com/rust-lang/rust/issues/79711))

A fallible function `f` applied to each element on array `self` in order to return an array the same size as `self` or the first error encountered.

The return type of this function depends on the return type of the closure. If you return `Result<T, E>` from the closure, you’ll get a `Result<[T; N], E>`. If you return `Option<T>` from the closure, you’ll get an `Option<[T; N]>`.

### [Examples](https://doc.rust-lang.org/nightly/std/primitive.array.html#examples-2)

```rs
#![feature(array_try_map)]
let a = ["1", "2", "3"];
let b = a.try_map(|v| v.parse::<u32>()).unwrap().map(|v| v + 1);
assert_eq!(b, [2, 3, 4]);

let a = ["1", "2a", "3"];
let b = a.try_map(|v| v.parse::<u32>());
assert!(b.is_err());

use std::num::NonZeroU32;
let z = [1, 2, 0, 3, 4];
assert_eq!(z.try_map(NonZeroU32::new), None);
let a = [1, 2, 3];
let b = a.try_map(NonZeroU32::new);
let c = b.map(|x| x.map(NonZeroU32::get));
assert_eq!(c, Some(a));
```
