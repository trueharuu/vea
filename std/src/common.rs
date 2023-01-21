use crate::complex::Complex;

#[derive(Clone)]
pub enum Number {
    Float(f64),
    Complex(Complex),
}

