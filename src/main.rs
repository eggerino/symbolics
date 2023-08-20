use std::ops;

#[derive(Debug)]
enum Expression {
    Constant(f64),
    Symbol(String),
    Function {
        name: String,
        args: Vec<Expression>,
    },
    Addition {
        augend: Box<Expression>,
        addend: Box<Expression>,
    },
    Subtraction {
        minuend: Box<Expression>,
        subtrahend: Box<Expression>,
    },
    Multiplication {
        multiplier: Box<Expression>,
        multiplicand: Box<Expression>,
    },
    Division {
        numerator: Box<Expression>,
        denominator: Box<Expression>,
    },
    Power {
        base: Box<Expression>,
        exponent: Box<Expression>,
    },
    Logarithm {
        base: Box<Expression>,
        anti_logarithm: Box<Expression>,
    },
    Sine(Box<Expression>),
    Cosine(Box<Expression>),
    SquareRoot(Box<Expression>),
}

impl ops::Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Addition {
            augend: Box::new(self),
            addend: Box::new(rhs),
        }
    }
}

impl ops::Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Subtraction {
            minuend: Box::new(self),
            subtrahend: Box::new(rhs),
        }
    }
}

impl ops::Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Multiplication {
            multiplier: Box::new(self),
            multiplicand: Box::new(rhs),
        }
    }
}

impl ops::Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Division {
            numerator: Box::new(self),
            denominator: Box::new(rhs),
        }
    }
}

impl ops::BitXor for Expression {
    type Output = Expression;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Expression::Power {
            base: Box::new(self),
            exponent: Box::new(rhs),
        }
    }
}

fn log(base: Expression, anti_logarithm: Expression) -> Expression {
    Expression::Logarithm { base: Box::new(base), anti_logarithm: Box::new(anti_logarithm) }
}

fn sin(arg: Expression) -> Expression {
    Expression::Sine(Box::new(arg))
}

fn cos(arg: Expression) -> Expression {
    Expression::Cosine(Box::new(arg))
}

fn sqrt(arg: Expression) -> Expression {
    Expression::SquareRoot(Box::new(arg))
}

fn main() {
    let m = Expression::Symbol(String::from("m"));
    let g = Expression::Constant(9.81);

    let f_g = m * g;

    let add_test = Expression::Constant(1.) + Expression::Constant(2.);
    let sub_test = Expression::Constant(1.) - Expression::Constant(2.);
    let mul_test = Expression::Constant(1.) * Expression::Constant(2.);
    let div_test = Expression::Constant(1.) / Expression::Constant(2.);
    let pow_test = Expression::Constant(1.) ^ Expression::Constant(2.);

    let sin_5 = sin(Expression::Constant(2.));
}
