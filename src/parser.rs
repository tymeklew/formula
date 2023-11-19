use crate::{error::FormulaError, formula::Equation};

pub trait Parser {
    fn from_str(&self, input: &'static str) -> Result<Equation, FormulaError>;
}

impl Parser for Equation {
    fn from_str(&self, input: &'static str) -> Result<Equation, FormulaError> {
        todo!()
    }
}
