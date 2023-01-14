use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    None,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Literal::Boolean(b) => b.to_string(),
            Literal::None => "None".to_string(),
            Literal::Number(d) => d.to_string(),
            Literal::String(s) => s.to_string()
        })
    }
}