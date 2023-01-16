use std::{ fmt::Display, ops::{ Div, Mul, Neg, Not, Sub }, sync::Mutex };

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
}

impl Literal {
    pub fn into_string(&self) -> String {
        match self {
            Literal::Boolean(b) => b.to_string(),
            Literal::None => "None".to_string(),
            Literal::Number(d) => d.to_string(),
            Literal::String(s) => s.clone(),
        }
    }

    pub fn into_number(&self) -> f64 {
        match self {
            Literal::Boolean(b) => {
                if *b { 1.0 } else { 0.0 }
            }
            Literal::None => f64::NAN,
            Literal::Number(d) => *d,
            Literal::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
        }
    }

    pub fn into_bool(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::None => false,
            Literal::Number(d) => !d.is_nan(),
            Literal::String(s) => true,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Literal::Boolean(b) => b.to_string(),
            Literal::None => "None".to_string(),
            Literal::Number(d) => d.to_string(),
            Literal::String(s) => s.to_string(),
        })
    }
}

impl Neg for Literal {
    type Output = f64;
    fn neg(self) -> Self::Output {
        -self.into_number()
    }
}

impl Not for Literal {
    type Output = bool;
    fn not(self) -> Self::Output {
        !self.into_bool()
    }
}

impl Sub<Literal> for Literal {
    type Output = f64;
    fn sub(self, rhs: Literal) -> Self::Output {
        self.into_number() - rhs.into_number()
    }
}

impl Div<Literal> for Literal {
    type Output = f64;
    fn div(self, rhs: Literal) -> Self::Output {
        self.into_number() / rhs.into_number()
    }
}

impl Mul<Literal> for Literal {
    type Output = f64;
    fn mul(self, rhs: Literal) -> Self::Output {
        self.into_number() * rhs.into_number()
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.into_number().partial_cmp(&other.into_number())
    }
}