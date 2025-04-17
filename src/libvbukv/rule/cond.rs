use std::fmt;
use std::str::FromStr;

/// Требование к букве.
///
/// Перечисление Cond содержит названия всех возможных требований к букве.
#[derive(Debug, Clone)]
pub enum Cond {
    /// Буква есть в слове.
    Plus(Option<usize>),
    /// Буквы нет в слове.
    Minus(Option<usize>),
    /// Буква есть только на указанной позиции и нигде больше.
    Equals(usize),
    /// Буква есть на любой позиции, кроме указанной.
    Asterisk(usize),
}

impl FromStr for Cond {
    type Err = ();

    fn from_str(s: &str) -> Result<Cond, Self::Err> {
        match s {
            "+" => Ok(Cond::Plus(None)),
            "-" => Ok(Cond::Minus(None)),
            "=" => Ok(Cond::Equals(1)),
            "*" => Ok(Cond::Asterisk(1)),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Cond {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cond::Plus(p) => format!("+{}", p.unwrap_or(0)),
            Cond::Minus(p) => format!("-{}", p.unwrap_or(0)),
            Cond::Equals(p) => format!("={}", p),
            Cond::Asterisk(p) => format!("*{}", p),
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_failed() -> Result<(), String> {
        let cond = Cond::from_str("^");

        assert!(cond.is_err());

        Ok(())
    }

    #[test]
    fn fmt_equals() {
        let cond = Cond::from_str("=").unwrap();
        assert_eq!(format!("{cond}"), "=");
    }

    #[test]
    fn fmt_asterisk() {
        let cond = Cond::from_str("*").unwrap();
        assert_eq!(format!("{cond}"), "*");
    }
}
