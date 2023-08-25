use super::*;

use super::BinaryOperator::*;
use super::Expression::*;
use super::UnaryOperator::*;

impl Expression {
    pub fn derive<F>(self, variable: &str, name_mutation: F) -> Self
    where
        F: Fn(String) -> String,
        F: Clone,
    {
        match self {
            Symbol(name) => derive_symbol(name, variable),
            Constant(_) => Constant(0.0),
            EulerNumber => Constant(0.0),
            UnaryOperation { operand, operator } => {
                derive_unary_operation(operand, operator, variable, name_mutation)
            }
            BinaryOperation {
                first_operand,
                second_operand,
                operator,
            } => derive_binary_operation(
                first_operand,
                second_operand,
                operator,
                variable,
                name_mutation,
            ),
            Function { name, arguments } => {
                derive_function(name, arguments, variable, name_mutation)
            }
            Sine(arg) => derive_sine(arg, variable, name_mutation),
            Cosine(arg) => derive_cosine(arg, variable, name_mutation),
        }
    }

    fn depends_on(&self, variable: &str) -> bool {
        match self {
            Symbol(name) => name == variable,
            Constant(_) => false,
            EulerNumber => false,
            UnaryOperation {
                operand,
                operator: _operator,
            } => operand.depends_on(variable),
            BinaryOperation {
                first_operand,
                second_operand,
                operator: _operator,
            } => first_operand.depends_on(variable) || second_operand.depends_on(variable),
            Function {
                name: _name,
                arguments,
            } => arguments.iter().any(|x| x.depends_on(variable)),
            Sine(arg) => arg.depends_on(variable),
            Cosine(arg) => arg.depends_on(variable),
        }
    }
}

fn derive_symbol(name: String, variable: &str) -> Expression {
    if name == variable {
        Constant(1.0)
    } else {
        Constant(0.0)
    }
}

fn derive_unary_operation<F>(
    operand: Box<Expression>,
    operator: UnaryOperator,
    variable: &str,
    name_mutation: F,
) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    match operator {
        Negation => UnaryOperation {
            operand: Box::new(operand.derive(variable, name_mutation)),
            operator: Negation,
        },
    }
}

fn derive_binary_operation<F>(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    operator: BinaryOperator,
    variable: &str,
    name_mutation: F,
) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    match operator {
        Addition => {
            first_operand.derive(variable, name_mutation.clone())
                + second_operand.derive(variable, name_mutation)
        }
        Subtraction => {
            first_operand.derive(variable, name_mutation.clone())
                - second_operand.derive(variable, name_mutation)
        }
        Multiplication => {
            first_operand
                .clone()
                .derive(variable, name_mutation.clone())
                * (*second_operand.clone())
                + (*first_operand) * second_operand.derive(variable, name_mutation)
        }
        Division => {
            (first_operand
                .clone()
                .derive(variable, name_mutation.clone())
                * (*second_operand.clone())
                - (*first_operand) * second_operand.clone().derive(variable, name_mutation))
                / ((*second_operand.clone()) * (*second_operand))
        }
        Power => derive_power(first_operand, second_operand, variable, name_mutation),
        Logarithm => derive_logarithm(first_operand, second_operand, variable, name_mutation),
    }
}

fn derive_power<F>(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    variable: &str,
    name_mutation: F,
) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    let df = first_operand
        .clone()
        .derive(variable, name_mutation.clone());
    let ds = second_operand.clone().derive(variable, name_mutation);

    let f = first_operand.clone().pow(*second_operand.clone());
    let ln_part = EulerNumber.log(*first_operand.clone()) * ds;
    let quot_part = df * (*second_operand) / (*first_operand);
    f * (ln_part + quot_part)
}

fn derive_logarithm<F>(
    first_operand: Box<Expression>,
    second_operand: Box<Expression>,
    variable: &str,
    name_mutation: F,
) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    let df = first_operand
        .clone()
        .derive(variable, name_mutation.clone());
    let ds = second_operand.clone().derive(variable, name_mutation);

    let ln_f = EulerNumber.log(*first_operand.clone());
    let ln_s = EulerNumber.log(*second_operand.clone());

    ((ds / (*second_operand)) * ln_f.clone() - ln_s * (df / (*first_operand.clone())))
        / (ln_f.clone() * ln_f)
}

fn derive_function<F>(
    name: String,
    arguments: Vec<Expression>,
    variable: &str,
    name_mutation: F,
) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    if arguments.iter().any(|x| x.depends_on(variable)) {
        Function {
            name: name_mutation(name),
            arguments,
        }
    } else {
        Constant(0.0)
    }
}

fn derive_sine<F>(arg: Box<Expression>, variable: &str, name_mutation: F) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    arg.clone().cos() * arg.derive(variable, name_mutation)
}

fn derive_cosine<F>(arg: Box<Expression>, variable: &str, name_mutation: F) -> Expression
where
    F: Fn(String) -> String,
    F: Clone,
{
    - arg.clone().sin() * arg.derive(variable, name_mutation)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn mutliplication() {
        let mut values = HashMap::new();
        let functions = HashMap::new();

        let a = sym!(a);
        let b = sym!(b);
        let prod = a * b;
        let der = prod.derive("a", |x| x);
        values.insert(String::from("a"), 1.0);
        values.insert(String::from("b"), 2.0);

        let result = der.eval(&values, &functions);

        assert!(result.is_some());
        assert!(result.unwrap() < 2.001);
        assert!(result.unwrap() > 1.000);
    }
}
