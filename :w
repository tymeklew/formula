use std::ops::{Add, Div, Mul, Sub};

pub trait Evaluate {
    fn evaluate(&self) -> f64;
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Literal(f64),
}

impl Evaluate for Node {
    fn evaluate(&self) -> f64 {
        match self {
            Node::Add(p1, p2) => p1.evaluate() + p2.evaluate(),
            Node::Subtract(p1, p2) => p1.evaluate() - p2.evaluate(),
            Node::Multiply(p1, p2) => p1.evaluate() * p2.evaluate(),
            Node::Divide(p1, p2) => p1.evaluate() / p2.evaluate(),
            Node::Power(p1, p2) => p1.evaluate().powf(p2.evaluate()),
            Node::Literal(p1) => *p1,
        }
    }
}

impl Add for Node {
    type Output = Node;
    fn add(self, rhs: Self) -> Self::Output {
        Node::Add(Box::new(self), Box::new(rhs))
    }
}
impl Sub for Node {
    type Output = Node;
    fn sub(self, rhs: Self) -> Self::Output {
        Node::Subtract(Box::new(self), Box::new(rhs))
    }
}
impl Mul for Node {
    type Output = Node;
    fn mul(self, rhs: Self) -> Self::Output {
        Node::Multiply(Box::new(self), Box::new(rhs))
    }
}
