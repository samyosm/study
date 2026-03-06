use crate::{
    parser::infix_to_postfix,
    tokenizer::{Function, Operator, Token, tokenize},
};

pub fn calculate_postfix(postfix: Vec<&Token>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in postfix {
        match token {
            Token::Number(a) => stack.push(*a),
            Token::Operator(op) => match op {
                Operator::Addition => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a + b);
                }
                Operator::Substraction => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a);
                }
                Operator::Multiplication => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);
                }
                Operator::Division => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);
                }
                Operator::Exponentiation => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b.powf(a));
                }
            },
            Token::Function(function) => match function {
                Function::Sin => {
                    let a = stack.pop().unwrap();
                    stack.push(a.sin());
                }
                Function::Cos => {
                    let a = stack.pop().unwrap();
                    stack.push(a.cos());
                }
                Function::Max => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a.max(b));
                }
                Function::Tan => {
                    let a = stack.pop().unwrap();
                    stack.push(a.tan());
                }
                Function::Ln => {
                    let a = stack.pop().unwrap();
                    stack.push(a.ln());
                }
                Function::Log => {
                    let a = stack.pop().unwrap();
                    stack.push(a.log10());
                }
            },
            _ => panic!("This shouldn't happen"),
        }
    }

    *stack.last().unwrap()
}

pub fn calculate(input: &str) -> f64 {
    let tokens = tokenize(input);
    let postfix = infix_to_postfix(&tokens);
    calculate_postfix(postfix)
}
