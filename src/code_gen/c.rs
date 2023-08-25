use super::*;

use super::super::BinaryOperator::*;
use super::super::Expression::*;
use super::super::UnaryOperator::*;

pub struct Options {
    pub anonymous_as_function: bool,
}

impl Expression {
    pub fn c_print(self, options: &Options) {
        match self {
            Symbol(name) => print!("{name}"),
            Constant(x) => print!("{x}"),
            EulerNumber => print!("2.71828182845904523536"),
            UnaryOperation { operand, operator } => print_unary_operation(operand, operator, options),
            BinaryOperation {
                first_operand,
                second_operand,
                operator,
            } => print_binary_operation(first_operand, second_operand, operator, options), 
            Function { name, arguments } => print_function(name, arguments, options),
            Sine(arg) => print_sine(arg, options),
            Cosine(arg) => print_cosine(arg, options),
        }
    }
}

fn print_unary_operation(operand: Box<Expression>, operator: UnaryOperator, options: &Options) {
    match operator {
        Negation => {
            print!("-(");
            operand.c_print(options);
            print!(")");
        }
    }
}

fn print_binary_operation(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    operator: BinaryOperator,
    options: &Options,
) {
    match operator {
        Addition => print_elementary_binary_operation(first_operand, second_operand, "+", options),
        Subtraction => print_elementary_binary_operation(first_operand, second_operand, "-", options),
        Multiplication => print_elementary_binary_operation(first_operand, second_operand, "*", options),
        Division => print_elementary_binary_operation(first_operand, second_operand, "/", options),
        Power => {
            print!("pow(");
            first_operand.c_print(options);
            print!(",");
            second_operand.c_print(options);
            print!(")");
        }
        Logarithm => {
            print!("log(");
            second_operand.c_print(options);
            print!(")/log(");
            first_operand.c_print(options);
            print!(")");
        }
    }
}

fn print_elementary_binary_operation(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    op: &str,
    options: &Options,
) {
    print!("(");
    first_operand.c_print(options);
    print!("){}(", op);
    second_operand.c_print(options);
    print!(")");
}

fn print_function(name: String, arguments: Vec<Expression>, options: &Options) {
    print!("{name}");
    if options.anonymous_as_function {
        print!("(");
        let mut is_first_arg = true;
        for arg in arguments {
            if is_first_arg {
                is_first_arg = false;
            } else {
                print!(",");
            }
            arg.c_print(options);
        }
        print!(")");
    }
}

fn print_sine(arg: Box<Expression>, options: &Options) {
    print!("sin(");
    arg.c_print(options);
    print!(")");
}

fn print_cosine(arg: Box<Expression>, options: &Options) {
    print!("cos(");
    arg.c_print(options);
    print!(")");
}