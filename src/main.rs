use symbolics::code_gen::c::Options;
use symbolics::*;

fn main() {
    let time = sym!(t);
    let radius = sym!(r);
    let angle = func!(angle, time);

    let x = radius * angle.sin();
    let dd_x_ddt = x
        .derive("t", |x| format!("d_{}", x))
        .derive("t", |x| format!("d_{}", x));

    let options = Options {
        anonymous_as_function: true,
    };
    print!("double result = ");
    dd_x_ddt.c_print(&options);
    print!(";\n");
}
