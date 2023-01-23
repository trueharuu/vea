#![allow(incomplete_features)]
#![feature(let_chains, generic_const_exprs)]
pub mod math_fns;
pub mod complex;
pub mod common;
pub mod imaginary;
pub mod rational;
pub mod set;
pub mod matrix;
pub mod consts;

fn main() {

}
#[allow(unused)]
macro_rules! set {
    ($($expression:expr),*) => {
      {
        let mut v = Vec::new();
        $(
          v.push($expression);
        )*
        Set(v)
      }
    };
}

#[cfg(test)]
pub mod tests {
    use crate::set::Set;

    #[test]
    fn set_union() {
        assert_eq!(set![1, 2] | set![2, 3], set![1, 2, 3]);
    }

    #[test]
    fn set_intersection() {
        assert_eq!(set![1, 2] & set![2, 3], set![2])
    }

    #[test]
    fn set_cartiesean_product() {
        assert_eq!(set![1, 2] * set![1, 2], set![(1, 1), (1, 2), (2, 1), (2, 2)])
    }

    #[test]
    fn set_add() {
        assert_eq!(set![1, 2] + 3, set![1, 2, 3])
    }

    #[test]
    fn set_remove() {
        assert_eq!(set![1, 2, 3] - 3, set![1, 2])
    }
}