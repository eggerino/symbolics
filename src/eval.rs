use super::*;

use std::collections::HashMap;

type ValueMap = HashMap<String, f64>;
type FunctionMap = HashMap<String, Box<dyn Fn(Vec<f64>) -> f64>>;

impl Expression {
    pub fn eval(self, values: &ValueMap, functions: &FunctionMap) -> Option<f64> {
        match self {
            Expression::Symbol(name) => values.get(&name).map(|x| x.clone()),
            Expression::Constant(x) => Some(x),
            Expression::EulerNumber => Some(2.71828182845904523536),
            Expression::UnaryOperation { operand, operator } => match operator {
                UnaryOperator::Negation => operand.eval(values, functions).map(|x| -x),
            },
            Expression::BinaryOperation {
                first_operand,
                second_operand,
                operator,
            } => eval_binary_operation(first_operand, second_operand, operator, values, functions),

            Expression::Function { name, arguments } => {
                eval_function(name, arguments, values, functions)
            }
            Expression::Sine(arg) => arg.eval(values, functions).map(|x| x.sin()),
            Expression::Cosine(arg) => arg.eval(values, functions).map(|x| x.cos()),
        }
    }
}

fn eval_binary_operation(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    operator: BinaryOperator,
    values: &ValueMap,
    functions: &FunctionMap,
) -> Option<f64> {
    let op = match operator {
        BinaryOperator::Addition => |a, b| a + b,
        BinaryOperator::Subtraction => |a, b| a - b,
        BinaryOperator::Multiplication => |a, b| a * b,
        BinaryOperator::Division => |a, b| a / b,
        BinaryOperator::Power => |a: f64, b| a.powf(b),
        BinaryOperator::Logarithm => |a: f64, b| a.log(b),
    };

    match first_operand.eval(values, functions) {
        None => None,
        Some(a) => match second_operand.eval(values, functions) {
            None => None,
            Some(b) => Some(op(a, b)),
        },
    }
}

fn eval_function(
    name: String,
    arguments: Vec<Expression>,
    values: &ValueMap,
    functions: &FunctionMap,
) -> Option<f64> {
    match functions.get(&name) {
        None => None,
        Some(function) => {
            let mut args = arguments.into_iter().map(|x| x.eval(values, functions));

            if args.any(|x| x.is_none()) {
                return None;
            } else {
                return Some(function(args.map(|x| x.unwrap()).collect()));
            }
        }
    }
}
