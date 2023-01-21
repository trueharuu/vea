use std::ops::{ Neg, Add, Sub, Mul, Div };

use crate::imaginary::i;

#[derive(Clone)]
pub struct Complex {
    pub r: f64,
    pub i: i,
}

impl Complex {
    pub fn new(r: f64, im: i) -> Self {
        Self { r, i: im }
    }

    pub fn as_r(&self) -> f64 {
        self.r
    }

    pub fn as_i(&self) -> i {
        self.i
    }

    pub fn i(im: i) -> Self {
        Self::new(0.0, im)
    }

    pub fn r(r: f64) -> Self {
        Self::new(r, i(0.0))
    }

    pub fn as_re(&self) -> (f64, i) {
        (self.r, self.i)
    }

    pub fn swap(&self) -> Self {
        Complex::new(self.i.real(), i(self.r))
    }

    pub fn conjugate(&self) -> Self {
        Complex::new(self.r, -self.i)
    }

    pub fn is_r(&self) -> bool {
        self.i.0 == 0.0
    }

    pub fn is_i(&self) -> bool {
        self.r == 0.0
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.r, -self.i)
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.i + rhs.i)
    }
}

impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.r + rhs, self.i)
    }
}

impl Add<i> for Complex {
    type Output = Self;
    fn add(self, rhs: i) -> Self::Output {
        Self::new(self.r, self.i + rhs)
    }
}

//

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.i - rhs.i)
    }
}

impl Sub<f64> for Complex {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.r - rhs, self.i)
    }
}

impl Sub<i> for Complex {
    type Output = Self;
    fn sub(self, rhs: i) -> Self::Output {
        Self::new(self.r, self.i - rhs)
    }
}

//

impl Mul for Complex {
    // (a + bi) * (c + di) = ac + bci + adi + bdii = ac + 2abcdi - bd
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r - self.i.0 * rhs.i.0, i(2.0 * self.r * self.i.0 * rhs.r * rhs.i.0))
    }
}

impl Mul<f64> for Complex {
    // (a + bi) * c = ca + cbi
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(rhs * self.r, self.i * rhs)
    }
}

impl Mul<i> for Complex {
    // (a + bi) * ci = aci - bc
    type Output = Self;
    fn mul(self, rhs: i) -> Self::Output {
        Self::new(-(self.i.0 * rhs.0), rhs * self.r)
    }
}

//

impl Div for Complex {
    // (a + bi) / (c + di) = (a / c) - (b / d)
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        self.r / rhs.r - self.i.0 / rhs.i.0
    }
}

impl Div<f64> for Complex {
    // (a + bi) / c = (a/c) + (b / c)i
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(rhs / self.r, i(self.i.0 / rhs))
    }
}

impl Div<i> for Complex {
    // (a + bi) / ci = (a/c)i - bc
    type Output = Self;
    fn div(self, rhs: i) -> Self::Output {
        Self::new(-(self.i.0 * rhs.0), rhs / self.r)
    }
}

//

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl PartialEq<f64> for Complex {
    fn eq(&self, other: &f64) -> bool {
        self.r == *other && self.i.0 == 0.0
    }
}

impl PartialEq<i> for Complex {
    fn eq(&self, other: &i) -> bool {
        self.i == *other && self.r == 0.0
    }
}

//

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_i() && other.is_i() {
            self.i.partial_cmp(&other.i)
        } else if self.is_r() && other.is_r() {
            self.r.partial_cmp(&other.r)
        } else {
            None
        }
    }
}

impl PartialOrd<f64> for Complex {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        if self.is_r() { self.r.partial_cmp(other) } else { None }
    }
}

impl PartialOrd<i> for Complex {
    fn partial_cmp(&self, other: &i) -> Option<std::cmp::Ordering> {
        if self.is_i() { self.i.partial_cmp(other) } else { None }
    }
}