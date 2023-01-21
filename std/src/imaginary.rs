use std::{ ops::{ Deref, Neg, Add, Sub, Mul, Div }, fmt::Display };

use crate::complex::Complex;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct i(pub f64);

impl i {
    pub fn real(&self) -> f64 {
        self.0
    }
}

impl Deref for i {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Neg for i {
    type Output = Self;
    fn neg(self) -> Self::Output {
        i(self.0)
    }
}

impl Add for i {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        i(self.0 + rhs.0)
    }
}

impl Add<f64> for i {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        Complex::new(rhs, self)
    }
}

impl Add<Complex> for i {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Complex::new(rhs.r, self + rhs.i)
    }
}

impl Sub for i {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        i(self.0 - rhs.0)
    }
}

impl Sub<f64> for i {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Self::Output {
        Complex::new(-rhs, self)
    }
}

impl Sub<Complex> for i {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::new(rhs.r, self - rhs.i)
    }
}

impl Mul for i {
    // ai * bi = (a * b)(i * i) = (a * b)(-1)
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * -rhs.0
    }
}

impl Mul<f64> for i {
    // a(bi) = (ab)i
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        i(self.0 * rhs)
    }
}

impl Mul<Complex> for i {
    // (a + bi) * ci = a*ci + bi*ci = a*ci - b = -b + a*ci
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Complex::new(-rhs.i.0, self * rhs.r)
    }
}

impl Div for i {
    // ai / bi = (a / b)(i / i) = (a * b)(1)
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}

impl Div<f64> for i {
    // a / bi = (a / b)i
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        i(self.0 / rhs)
    }
}

impl Div<Complex> for i {
    // (a + bi) / ci = a/c + (b/c)i
    type Output = Complex;
    fn div(self, rhs: Complex) -> Self::Output {
        Complex::new(rhs.r / self.0, i(*rhs.i / self.0))
    }
}

impl Display for i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}i", self.0)
    }
}

impl PartialEq<f64> for i {
    fn eq(&self, other: &f64) -> bool {
        self.0 == 0.0 && *other == 0.0
    }
}

impl PartialEq<Complex> for i {
    fn eq(&self, other: &Complex) -> bool {
        *self == other.i && other.r == 0.0
    }
}

impl PartialOrd<f64> for i {
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Complex> for i {
    fn partial_cmp(&self, other: &Complex) -> Option<std::cmp::Ordering> {
        if other.is_i() { self.partial_cmp(&other.i) } else { None }
    }
}