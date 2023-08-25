use symbolics::Expression;
use std::collections::HashMap;

fn main() {
    let m = Expression::Symbol(String::from("m"));
    let g = Expression::Constant(9.81);

    let f_g = m * g;

    let mut values = HashMap::new();
    values.insert(String::from("m"), 10.0);

    let functions = HashMap::new();
    let result = f_g.eval(&values, &functions);
    if let Some(x) = result {
        println!("result = {x}");
    }

    let a = Expression::Symbol(String::from("a"));
    let b = Expression::Symbol(String::from("b"));
    let prod = a * b;
    let der = prod.derive("a", |x| x);
    values.insert(String::from("a"), 1.0);
    values.insert(String::from("b"), 2.0);

    let der_result = der.eval(&values, &functions);
    if let Some(x) = der_result {
        println!("der_result = {x}");
    }

}
