use super::*;

use std::collections::HashMap;

impl Expression {
    pub fn eval(self, values: &HashMap<String, f64>) -> Option<f64> {
        match self {
            Expression::Symbol(name) => values.get(&name).map(|x| x.clone()),
            Expression::Constant(x) => Some(x),
            Expression::UnaryOperation { operand, operator } => match operator {
                UnaryOperator::Negation => operand.eval(&values).map(|x| -x),
            },
            Expression::BinaryOperation {
                first_operand,
                second_operand,
                operator,
            } => match operator {
                BinaryOperator::Addition => match first_operand.eval(&values) {
                    Some(a) => match second_operand.eval(&values) {
                        Some(b) => Some(a + b),
                        None => None,
                    },
                    None => None,
                },
                BinaryOperator::Subtraction => binary_operation(first_operand, second_operand, &values, |a, b| a - b),
                BinaryOperator::Multiplication => binary_operation(first_operand, second_operand, &values, |a, b| a * b),
                BinaryOperator::Division => binary_operation(first_operand, second_operand, &values, |a, b| a / b),
                BinaryOperator::Power => binary_operation(first_operand, second_operand, &values, |a, b| a.powf(b)),
                BinaryOperator::Logarithm => binary_operation(first_operand, second_operand, &values, |a, b| a.log(b)),
            },
            Expression::Function { name: _name, arguments: _arguments } => todo!(),
            Expression::Sine(arg) => arg.eval(values).map(|x| x.sin()),
            Expression::Cosine(arg) => arg.eval(values).map(|x| x.cos()),
        }
    }
}

fn binary_operation<F>(first_operand: Box<Expression>, second_operand: Box<Expression>, values: &HashMap<String, f64>, op: F) -> Option<f64> where
    F: Fn(f64, f64) -> f64 {
    match first_operand.eval(&values) {
        None => None,
        Some(a) => match second_operand.eval(&values) {
            None => None,
            Some(b) => Some(op(a, b)),
        }
    }
}
