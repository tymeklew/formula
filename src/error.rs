use std::num::ParseFloatError;

#[derive(Debug)]
pub enum FormulaError {
    InvalidFormula,
    InvalidOperator,
    NoEqualSign,
    NoVariable,
    ParseFloatError(ParseFloatError),
}

impl From<ParseFloatError> for FormulaError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl std::fmt::Display for FormulaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaError::InvalidFormula => write!(f, "Invalid Formula"),
            FormulaError::InvalidOperator => write!(f, "Invalid Operator"),
            FormulaError::NoEqualSign => write!(f, "No Equal Sign"),
            FormulaError::NoVariable => write!(f, "No Variable"),
            FormulaError::ParseFloatError(_) => write!(f, "Parse Float Error"),
        }
    }
}

impl std::error::Error for FormulaError {}
