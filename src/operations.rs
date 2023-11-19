use crate::Evaluate;
use std::fmt::Display;

pub trait Inverse {
    fn inverse(&self) -> Self;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Operators {
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
    Power(Box<Self>, Box<Self>), // (base , value)
    Log(Box<Self>, Box<Self>),   // (base , value)
    Root(Box<Self>, Box<Self>),  // (base , value)
    Constant(f64),
    Variable,
}

impl Evaluate for Operators {
    fn evaluate(&self) -> f64 {
        match self {
            Operators::Add(left, right) => left.evaluate() + right.evaluate(),
            Operators::Subtract(left, right) => left.evaluate() - right.evaluate(),
            Operators::Multiply(left, right) => left.evaluate() * right.evaluate(),
            Operators::Divide(left, right) => left.evaluate() / right.evaluate(),
            Operators::Power(left, right) => left.evaluate().powf(right.evaluate()),
            Operators::Log(left, right) => right.evaluate().log(left.evaluate()),
            Operators::Root(left, right) => right.evaluate().powf(1.0 / left.evaluate()),
            Operators::Constant(value) => *value,
            Operators::Variable => panic!("Cannot evaluate a variable"),
        }
    }
    fn contains_var(&self) -> bool {
        match self {
            Self::Variable => return true,
            Self::Add(left, right) => return left.contains_var() || right.contains_var(),
            Self::Subtract(left, right) => return left.contains_var() || right.contains_var(),
            Self::Multiply(left, right) => return left.contains_var() || right.contains_var(),
            Self::Divide(left, right) => return left.contains_var() || right.contains_var(),
            Self::Power(left, right) => return left.contains_var() || right.contains_var(),
            Self::Log(left, right) => return left.contains_var() || right.contains_var(),
            Self::Root(left, right) => return left.contains_var() || right.contains_var(),
            Self::Constant(_) => return false,
        }
    }
}

impl Display for Operators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operators::Add(left, right) => write!(f, "({} + {})", left, right),
            Operators::Subtract(left, right) => write!(f, "({} - {})", left, right),
            Operators::Multiply(left, right) => write!(f, "({} * {})", left, right),
            Operators::Divide(left, right) => write!(f, "({} / {})", left, right),
            Operators::Power(left, right) => write!(f, "({} ^ {})", left, right),
            Operators::Log(left, right) => write!(f, "log {}({})", left, right),
            Operators::Root(left, right) => write!(f, "root {}({})", left, right),
            Operators::Constant(value) => write!(f, "{}", value),
            Operators::Variable => write!(f, "x"),
        }
    }
}
