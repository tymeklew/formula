use crate::{operations::Operators, Evaluate};

pub trait Formula {
    fn rearrange(&self) -> Self;
}

// Represent the left and right hand side of the formula
pub type Equation = (Operators, Operators);

impl Formula for Equation {
    fn rearrange(&self) -> Self {
        // Example (x + 5) = ((2 * x) - 16);
        let (mut left, mut right) = self.clone();

        // First move the variable to the left hand side
        loop {
            println!("{} = {}\n", left, right);

            if let Operators::Variable = left {
                break;
            }

            match left {
                Operators::Add(ref l, ref r) => {
                    if l.contains_var() {
                        let temp = r.clone();
                        left = *l.clone();
                        right = Operators::Subtract(Box::new(right.clone()), temp);
                    } else {
                        let temp = l.clone();
                        left = *r.clone();
                        right = Operators::Subtract(Box::new(right.clone()), temp);
                    }
                }
                Operators::Subtract(ref l, ref r) => {
                    if l.contains_var() {
                        let temp = r.clone();
                        left = *l.clone();
                        right = Operators::Add(Box::new(right.clone()), temp);
                    } else {
                        let temp = l.clone();
                        left = *r.clone();
                        right = Operators::Add(Box::new(right.clone()), temp);
                    }
                }
                Operators::Multiply(ref l, ref r) => {
                    if l.contains_var() {
                        let temp = r.clone();
                        left = *l.clone();
                        right = Operators::Divide(Box::new(right.clone()), temp);
                    } else {
                        let temp = l.clone();
                        left = *r.clone();
                        right = Operators::Divide(Box::new(right.clone()), temp);
                    }
                }
                Operators::Divide(ref l, ref r) => {
                    if l.contains_var() {
                        let temp = r.clone();
                        left = *l.clone();
                        right = Operators::Multiply(Box::new(right.clone()), temp);
                    } else {
                        let temp = l.clone();
                        left = *r.clone();
                        right = Operators::Multiply(Box::new(right.clone()), temp);
                    }
                }
                Operators::Power(ref l, ref r) => {
                    if l.contains_var() {
                        // x ^ 2
                        let temp = r.clone();
                        left = *l.clone();
                        right = Operators::Root(temp, Box::new(right.clone()))
                    } else {
                        // 2 ^ x
                        let temp = l.clone();
                        left = *r.clone();
                        right = Operators::Log(temp, Box::new(right.clone()));
                    }
                }
                _ => todo!(),
            }
        }

        return (left, right);
    }
}
