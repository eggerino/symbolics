use std::ops;

pub mod eval;
pub mod derivative;
pub mod code_gen;
pub mod transform;

#[derive(Clone)]
pub enum UnaryOperator {
    Negation,
}

#[derive(Clone)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
    Logarithm,
}

#[derive(Clone)]
pub enum Expression {
    Symbol(String),
    Constant(f64),
    EulerNumber,
    UnaryOperation {
        operand: Box<Expression>,
        operator: UnaryOperator,
    },
    BinaryOperation {
        first_operand: Box<Expression>,
        second_operand: Box<Expression>,
        operator: BinaryOperator,
    },
    Function {
        name: String,
        arguments: Vec<Expression>,
    },
    Sine(Box<Expression>),
    Cosine(Box<Expression>),
}

impl ops::Neg for Expression {
    type Output = Expression;

    fn neg(self) -> Self::Output {
        Expression::UnaryOperation { operand: Box::new(self), operator: UnaryOperator::Negation }
    }
}

impl ops::Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(rhs),
            operator: BinaryOperator::Addition,
        }
    }
}

impl ops::Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(rhs),
            operator: BinaryOperator::Subtraction,
        }
    }
}

impl ops::Mul for Expression {
    type Output = Expression;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(rhs),
            operator: BinaryOperator::Multiplication,
        }
    }
}

impl ops::Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(rhs),
            operator: BinaryOperator::Division,
        }
    }
}

impl Expression {
    pub fn pow(self, exponent: Self) -> Self {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(exponent),
            operator: BinaryOperator::Power,
        }
    }

    pub fn log(self, base: Self) -> Self {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(base),
            operator: BinaryOperator::Logarithm,
        }
    }
    pub fn sin(self) -> Self {
        Expression::Sine(Box::new(self))
    }

    pub fn cos(self) -> Self {
        Expression::Cosine(Box::new(self))
    }

    pub fn sqrt(self) -> Self {
        Expression::BinaryOperation {
            first_operand: Box::new(self),
            second_operand: Box::new(Expression::Constant(1.0) / Expression::Constant(2.0)),
            operator: BinaryOperator::Power,
        }
    }
}

#[macro_export]
macro_rules! val {
    ($value: expr) => {
        Expression::Constant($value)
    };
}

#[macro_export]
macro_rules! sym {
    ($name: expr) => {
        Expression::Symbol(String::from(stringify!($name)))
    };
}

#[macro_export]
macro_rules! func {
    ($name: expr, $($args:expr), + $(,)?) => {
        Expression::Function { name: String::from(stringify!($name)), arguments: vec![$($args),+] }
    }
}

#[macro_export]
macro_rules! ln {
    ($arg: expr) => {
        Expression::EulerNumber.log($arg)
    };
}