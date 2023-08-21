use symbolics::Expression;
use std::collections::HashMap;

fn main() {
    let m = Expression::Symbol(String::from("m"));
    let g = Expression::Constant(9.81);

    let f_g = m * g;

    let mut values = HashMap::new();
    values.insert(String::from("m"), 10.0);

    let result = f_g.eval(&values);
    if let Some(x) = result {
        println!("result = {x}");
    }
}
