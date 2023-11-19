mod error;
mod formula;
mod operations;
mod parser;

pub trait Evaluate {
    fn evaluate(&self) -> f64;
    fn contains_var(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::formula::{Equation, Formula};
    use crate::operations::Operators;
    use crate::Evaluate;

    #[test]
    fn construct() {}

    #[test]
    fn solver() {
        // 2x + 5 = 10
        let left = Operators::Add(
            Box::new(Operators::Multiply(
                Box::new(Operators::Constant(2.0)),
                Box::new(Operators::Variable),
            )),
            Box::new(Operators::Constant(5.0)),
        );
        let right = Operators::Constant(10.0);

        let equation: Equation = (left, right);

        let x = equation.rearrange();
        assert_eq!(x.1.evaluate(), 2.5);

        // 5x / 10 = 25
        let left = Operators::Divide(
            Box::new(Operators::Multiply(
                Box::new(Operators::Constant(5.0)),
                Box::new(Operators::Variable),
            )),
            Box::new(Operators::Constant(10.0)),
        );
        let right = Operators::Constant(25.0);
        let equation: Equation = (left, right);
        let x = equation.rearrange();
        assert_eq!(x.1.evaluate(), 50.0);

        // 2^x = 16
        let left = Operators::Power(
            Box::new(Operators::Constant(2.0)),
            Box::new(Operators::Variable),
        );
        let right = Operators::Constant(16.0);
        let equation: Equation = (left, right);
        let x = equation.rearrange();
        assert_eq!(x.1.evaluate(), 4.0);

        // x^2 = 64
        let left = Operators::Power(
            Box::new(Operators::Variable),
            Box::new(Operators::Constant(2.0)),
        );
        let right = Operators::Constant(64.0);
        let equation: Equation = (left, right);
        let x = equation.rearrange();
        assert_eq!(x.1.evaluate(), 8.0);

        // 2x^2 - 10 = 8
        let left = Operators::Subtract(
            Box::new(Operators::Multiply(
                Box::new(Operators::Constant(2.0)),
                Box::new(Operators::Power(
                    Box::new(Operators::Variable),
                    Box::new(Operators::Constant(2.0)),
                )),
            )),
            Box::new(Operators::Constant(10.0)),
        );
        let right = Operators::Constant(8.0);
        let equation: Equation = (left, right);
        let x = equation.rearrange();
        assert_eq!(x.1.evaluate(), 3.0);
    }
}
/*
 *  let formula = formala!(x + y = 10)
 *
 *  formula.solve('x' , 5);
 *
 * */
