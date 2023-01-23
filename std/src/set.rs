use std::{
    ops::{ Add, Sub, BitOr, BitAnd, Div, Mul, Deref, Not, Shr, Shl, BitXor },
    cmp::Ordering::{ Equal, self },
    iter::once,
};

#[derive(Clone, Debug)]
pub struct Set<T: Clone + PartialEq>(pub Vec<T>);

impl<T> Set<T> where T: PartialEq + Clone {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn vec(&self) -> &Vec<T> {
        &self.0
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.vec().into_iter()
    }

    pub fn has(&self, value: &T) -> bool {
        self.0.contains(value)
    }

    pub fn push(&mut self, value: T) -> &mut Self {
        if !self.has(&value) {
            self.0.push(value);
        }

        self
    }

    pub fn remove(&mut self, value: T) -> &mut Self {
        let index = self.0
            .iter()
            .enumerate()
            .find(|(_, v)| &&value == v);

        if let Some((i, _)) = index {
            self.0.swap_remove(i);
        }

        self
    }

    pub fn union(&self, u: Set<T>) -> Set<T> {
        let mut out = Set::new();

        for i in self.iter().cloned() {
            out.push(i);
        }

        for i in u.iter().cloned() {
            out.push(i);
        }

        out
    }

    pub fn set_difference(&self, u: Set<T>) -> Set<T> {
        let mut out = Set::new();

        for i in self.iter().cloned() {
            if !u.has(&i) {
                out.push(i);
            }
        }

        out
    }

    pub fn intersection(&self, u: Set<T>) -> Set<T> {
        let mut out = Set::new();

        for i in self.iter().cloned() {
            if u.has(&i) {
                out.push(i);
            }
        }

        out
    }

    pub fn symmetric_difference(&self, u: Set<T>) -> Set<T> {
        Self::union(&self.set_difference(u.clone()), u.set_difference(self.clone()))
    }

    pub fn cartesian_product(&self, u: Set<T>) -> Set<(T, T)> {
        let mut out = Set::new();
        for i in self.iter().cloned() {
            for j in u.iter().cloned() {
                out.push((i.clone(), j));
            }
        }

        out
    }

    pub fn is_superset(&self, u: Set<T>) -> bool {
        for i in u.iter().cloned() {
            if !self.has(&i) {
                return false;
            }
        }

        true
    }

    pub fn is_subset(&self, u: Set<T>) -> bool {
        u.is_superset(self.clone())
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn is_null(&self) -> bool {
        self.size() == 0
    }

    pub fn map<U, F>(&self, f: F) -> Set<U> where F: FnMut(&T) -> U, U: Clone + PartialEq {
        Set(self.0.iter().map(f).collect::<Vec<_>>())
    }

    pub fn powerset(&self) -> Set<Set<T>> {
        self.iter().fold(Set(Vec::from_iter(once(Set(Vec::new())))), |x, i|
            x.map(|x| x.clone() + i.clone()).union(x)
        )
    }
}

impl<T> IntoIterator for Set<T> where T: Clone + PartialEq {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// { 1, 2 } + 3 = { 1, 2, 3 }
impl<T> Add<T> for Set<T> where T: Clone + PartialEq {
    type Output = Set<T>;
    fn add(mut self, rhs: T) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

// { 1, 2 } - 2 = { 1 }
impl<T> Sub<T> for Set<T> where T: Clone + PartialEq {
    type Output = Set<T>;
    fn sub(mut self, rhs: T) -> Self::Output {
        self.remove(rhs);
        self
    }
}

impl<T> Mul for Set<T> where T: Clone + PartialEq {
    type Output = Set<(T, T)>;
    fn mul(self, rhs: Self) -> Self::Output {
        self.cartesian_product(rhs)
    }
}

// { 1, 2, 3 } / { 2, 3 } = { 1 }
impl<T> Div for Set<T> where T: Clone + PartialEq {
    type Output = Set<T>;
    fn div(self, rhs: Self) -> Self::Output {
        self.set_difference(rhs)
    }
}

// { 1, 2 } | { 2, 3 } = { 1, 2, 3 }
impl<T> BitOr for Set<T> where T: Clone + PartialEq {
    type Output = Set<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

// { 1, 2 } & { 2, 3 } = { 2 }
impl<T> BitAnd for Set<T> where T: Clone + PartialEq {
    type Output = Set<T>;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<T> PartialEq for Set<T> where T: Clone + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        (self.clone() / other.clone()).is_null()
    }
}

impl<T> Eq for Set<T> where T: Clone + PartialEq {}
impl<T> PartialOrd for Set<T> where T: Clone + PartialEq {
    // where self is a superset of other, or if self is other
    fn ge(&self, other: &Self) -> bool {
        (self == other) | self.is_superset(other.clone())
    }

    // where self is a superset of other, but not when self is other
    fn gt(&self, other: &Self) -> bool {
        (self != other) & self.is_superset(other.clone())
    }

    // where self is a subset of other, or if self is other
    fn le(&self, other: &Self) -> bool {
        (self == other) | self.is_subset(other.clone())
    }

    // where self is a subset of other, but not when self is other
    fn lt(&self, other: &Self) -> bool {
        (self != other) & self.is_subset(other.clone())
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Equal)
        } else if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl<T> Not for Set<T> where T: PartialEq + Clone {
    type Output = Exclude<T>;
    fn not(self) -> Self::Output {
        Exclude(self)
    }
}

pub struct Exclude<T: PartialEq + Clone>(Set<T>);

impl<T> Deref for Exclude<T> where T: PartialEq + Clone {
    type Target = Set<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Exclude<T> where T: PartialEq + Clone {
    pub fn has(&self, value: &T) -> bool {
        !self.0.has(value)
    }
}

impl<T> Not for Exclude<T> where T: PartialEq + Clone {
    type Output = Set<T>;
    fn not(self) -> Self::Output {
        self.0
    }
}

impl<T> Shr<T> for Set<T> where T: PartialEq + Clone {
    type Output = bool;
    fn shr(self, rhs: T) -> Self::Output {
        self.has(&rhs)
    }
}

impl<T> Shl<T> for Set<T> where T: PartialEq + Clone {
    type Output = bool;
    fn shl(self, rhs: T) -> Self::Output {
        !self.has(&rhs)
    }
}

impl<T> BitXor for Set<T> where T: PartialEq + Clone {
    type Output = Set<T>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}