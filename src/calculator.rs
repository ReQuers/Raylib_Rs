use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    InvalidInput,
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidInput => write!(f, "invalid input"),
            CalcError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

pub fn calculate(expr: &str) -> Result<f64, CalcError> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Err(CalcError::InvalidInput);
    }

    let (idx, op) = trimmed
        .char_indices()
        .skip(1)
        .find_map(|(i, c)| Operation::from_char(c).map(|op| (i, op)))
        .ok_or(CalcError::InvalidInput)?;

    let left_str = trimmed[..idx].trim();
    let right_str = trimmed[idx + 1..].trim();

    let num1: f64 = left_str.parse().map_err(|_| CalcError::InvalidInput)?;
    let num2: f64 = right_str.parse().map_err(|_| CalcError::InvalidInput)?;

    match op {
        Operation::Add => Ok(num1 + num2),
        Operation::Subtract => Ok(num1 - num2),
        Operation::Multiply => Ok(num1 * num2),
        Operation::Divide => {
            if num2 == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(num1 / num2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        assert_eq!(calculate("2+3"), Ok(5.0));
        assert_eq!(calculate("10 - 4"), Ok(6.0));
        assert_eq!(calculate("3 * 4"), Ok(12.0));
        assert_eq!(calculate("9/3"), Ok(3.0));
    }

    #[test]
    fn negative_first_number() {
        assert_eq!(calculate("-5+2"), Ok(-3.0));
    }

    #[test]
    fn errors() {
        assert_eq!(calculate(""), Err(CalcError::InvalidInput));
        assert_eq!(calculate("5"), Err(CalcError::InvalidInput));
        assert_eq!(calculate("abc+1"), Err(CalcError::InvalidInput));
        assert_eq!(calculate("1/0"), Err(CalcError::DivisionByZero));
    }
}
