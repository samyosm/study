use crate::tokenizer::{Assocativity, Token};

pub fn infix_to_postfix(infix: &[Token]) -> Vec<&Token> {
    let mut output: Vec<&Token> = Vec::new();
    let mut operators: Vec<&Token> = Vec::new();
    let iter = infix.iter();
    for token in iter {
        match token {
            Token::Number(_) => output.push(token),
            Token::Function(_) => operators.push(token),
            Token::Comma => {
                while let Some(op) = operators.pop_if(|op| !matches!(op, Token::LeftParenthesis)) {
                    output.push(op);
                }
            }
            Token::Operator(o1) => {
                while let Some(last_op) = operators.last() {
                    if !matches!(last_op, Token::LeftParenthesis) {
                        if let Token::Operator(o2) = last_op {
                            if o2.precedence() > o1.precedence()
                                || (o1.precedence() == o2.precedence()
                                    && matches!(o1.assocativity(), Assocativity::Left))
                            {
                                output.push(operators.pop().unwrap());
                                continue;
                            }
                        }
                    }
                    break;
                }
                operators.push(token);
            }
            Token::LeftParenthesis => operators.push(token),
            Token::RightParenthsis => {
                while let Some(last_op) =
                    operators.pop_if(|op| !matches!(op, Token::LeftParenthesis))
                {
                    output.push(last_op);
                    assert!(!operators.is_empty(), "Mismatched parenthesis: 1");
                }

                assert!(
                    matches!(operators.pop(), Some(Token::LeftParenthesis)),
                    "Mismatched parenthesis: 2"
                );

                if let Some(Token::Function(_)) = operators.last() {
                    output.push(operators.pop().unwrap());
                }
            }
        }
    }

    while let Some(token) = operators.pop() {
        assert!(
            !matches!(token, Token::LeftParenthesis),
            "Mismatched parenthesis: 3"
        );

        output.push(token);
    }

    output
}
