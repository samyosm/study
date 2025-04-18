use std::{
    f64::consts::{E, PI},
    str::FromStr,
};

use strum::EnumString;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Function(Function),
    Operator(Operator),
    LeftParenthesis,
    RightParenthsis,
    Comma,
}

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Max,
    Ln,
    Log,
}

#[derive(Debug)]
pub enum Operator {
    Addition,
    Substraction,
    Division,
    Multiplication,
    Exponentiation,
}

#[derive(Debug)]
pub enum Assocativity {
    Left,
    Right,
}

impl Operator {
    pub fn precedence(&self) -> usize {
        match *self {
            Operator::Addition => 2,
            Operator::Substraction => 2,
            Operator::Division => 3,
            Operator::Multiplication => 3,
            Operator::Exponentiation => 4,
        }
    }

    pub fn assocativity(&self) -> Assocativity {
        match *self {
            Operator::Addition => Assocativity::Left,
            Operator::Substraction => Assocativity::Left,
            Operator::Division => Assocativity::Left,
            Operator::Multiplication => Assocativity::Left,
            Operator::Exponentiation => Assocativity::Right,
        }
    }
}

pub fn tokenize(infix: &str) -> Vec<Token> {
    let mut output = Vec::new();
    let mut chars = infix.chars().peekable();
    while let Some(char) = chars.next() {
        match char {
            ' ' => (),
            '(' => output.push(Token::LeftParenthesis),
            ')' => output.push(Token::RightParenthsis),
            ',' => output.push(Token::Comma),
            '*' => output.push(Token::Operator(Operator::Multiplication)),
            '+' => output.push(Token::Operator(Operator::Addition)),
            '-' => output.push(Token::Operator(Operator::Substraction)),
            '/' => output.push(Token::Operator(Operator::Division)),
            '^' => output.push(Token::Operator(Operator::Exponentiation)),
            // TODO: Support negative numbers? E.g., -25, ...
            // TODO: Support constants E.g., pi, e, ...
            '.' | '0'..='9' => {
                let mut num_str = String::from(char);
                while let Some(number) = chars.next_if(|&ch| ch.is_ascii_digit() || ch == '.') {
                    num_str.push(number);
                }
                let num: f64 = num_str.parse().unwrap();
                output.push(Token::Number(num));
            }
            // TODO: Support uppercase function names
            'a'..='z' => {
                let mut name = String::from(char);
                while let Some(character) = chars.next_if(|&ch| ch.is_ascii_lowercase()) {
                    name.push(character);
                }

                match name.as_str() {
                    "pi" => output.push(Token::Number(PI)),
                    "e" => output.push(Token::Number(E)),
                    _ => output.push(Token::Function(Function::from_str(&name).unwrap())),
                }
            }
            _ => panic!("Unkonwn character"),
        }
    }
    output
}
