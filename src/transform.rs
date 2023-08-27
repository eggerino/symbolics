use super::*;

use super::Expression::*;

impl Expression {
    pub fn transform<F, M>(&mut self, find: F, mutation: M)
    where
        F: Fn(&Expression) -> bool,
        F: Clone,
        M: Fn(&mut Expression),
        M: Clone,
    {
        if find(self) {
            mutation(self);
        }

        match self {
            Symbol(_) => (),
            Constant(_) => (),
            EulerNumber => (),
            UnaryOperation { operand, operator: _operator } => operand.transform(find, mutation),
            BinaryOperation { first_operand, second_operand, operator: _operator } => {
                first_operand.transform(find.clone(), mutation.clone());
                second_operand.transform(find, mutation);
            },
            Function { name: _name, arguments } => {
                for arg in arguments {
                    arg.transform(find.clone(), mutation.clone());
                }
            },
            Sine(arg) => arg.transform(find, mutation),
            Cosine(arg) => arg.transform(find, mutation),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn change_constant_to_euler_number() {
        let mut x = val!(69.420);

        x.transform(|x| matches!(x, Constant(_)), |x| {
            *x = EulerNumber;
        });

        assert!(matches!(x, EulerNumber));
    }
}