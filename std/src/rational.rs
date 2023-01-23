use std::{fmt::{ Display, Debug }};

use crate::math_fns::gcd;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rational {
    n: i32,
    d: i32,
}

impl Rational {
    pub fn new(n: i32, d: i32) -> Self {
        (Self { n, d }).reduce()
    }

    fn reduce(&self) -> Self {
        let gcd = gcd(self.n as f64, self.d as f64) as i32;
        Self { n: self.n / gcd, d: self.d / gcd }
    }

    pub fn from_float(float: f64) -> Self {
        let len = float.to_string().len();
        let mut d = (10.0f64).powi(len as i32);
        let mut n = float * d;
        let v = gcd(n, d);

        n /= v;
        d /= v;

        Self { n: n as i32, d: d as i32 }
    }
}

impl Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.n, self.d)
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unicode_frac(self.n, self.d))
    }
}

fn superscripts() -> [char; 10] {
    ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹']
}
fn subscripts() -> [char; 10] {
    ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉']
}
fn basic_frac(n: i32, d: i32) -> String {
    if n == d {
        return "1".to_string();
    }
    if d == 1 {
        return n.to_string();
    }
    n
        .to_string()
        .chars()
        .map(|x| superscripts()[x.to_digit(10).unwrap() as usize])
        .collect::<String>() +
        "/" +
        &d
            .to_string()
            .chars()
            .map(|x| subscripts()[x.to_digit(10).unwrap() as usize])
            .collect::<String>()
}

fn unicode_frac(n: i32, d: i32) -> String {
    match n {
        1 =>
            match d {
                2 => "\u{00bd}".to_string(),
                3 => "\u{2153}".to_string(),
                4 => "\u{00bc}".to_string(),
                5 => "\u{2155}".to_string(),
                6 => "\u{2159}".to_string(),
                7 => "\u{2150}".to_string(),
                8 => "\u{215b}".to_string(),
                9 => "\u{2151}".to_string(),
                10 => "\u{2152}".to_string(),
                _ => basic_frac(n, d),
            }
        2 =>
            match d {
                3 => "\u{2154}".to_string(),
                5 => "\u{2156}".to_string(),
                _ => basic_frac(n, d),
            }
        3 =>
            match d {
                4 => "\u{00be}".to_string(),
                5 => "\u{2157}".to_string(),
                8 => "\u{215c}".to_string(),
                _ => basic_frac(n, d),
            }
        4 if d == 5 => "\u{2158}".to_string(),
        5 =>
            match d {
                6 => "\u{215a}".to_string(),
                8 => "\u{215d}".to_string(),
                _ => basic_frac(n, d),
            }
        7 if d == 8 => "\u{215e}".to_string(),
        0 if d == 3 => "\u{2189}".to_string(),
        _ => basic_frac(n, d),
    }
}