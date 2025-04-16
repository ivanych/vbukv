use std::fmt;
use std::str::FromStr;

/// Требование к букве.
///
/// Перечисление Cond содержит названия всех возможных требований к букве.
#[derive(Debug, Clone)]
pub enum Cond {
    /// Буква есть в слове.
    Plus,
    /// Буквы нет в слове.
    Minus,
    /// Буква есть только на указанной позиции и нигде больше.
    Equals,
    /// Буква есть на любой позиции, кроме указанной.
    Asterisk,
}

impl FromStr for Cond {
    type Err = ();

    fn from_str(s: &str) -> Result<Cond, Self::Err> {
        match s {
            "+" => Ok(Cond::Plus),
            "-" => Ok(Cond::Minus),
            "=" => Ok(Cond::Equals),
            "*" => Ok(Cond::Asterisk),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Cond {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Cond::Plus => "+",
            Cond::Minus => "-",
            Cond::Equals => "=",
            Cond::Asterisk => "*",
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
