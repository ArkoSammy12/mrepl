use std::collections::vec_deque;
use std::fmt::{Display, Formatter};
use crate::lexer::{Token, Operator};

#[derive(Debug)]
pub struct Ast {
    pub root: AstNode
}

#[derive(Debug)]
pub enum AstNode {
    Operand {
        value: f64
    },
    BinaryOperation {
        operator: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryOperation {
        operator: UnaryOperator,
        value: Box<AstNode>
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power
}

impl Display for BinaryOperator {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Power => "^"
        })
    }

}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Negate,
}

impl Display for UnaryOperator {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnaryOperator::Plus => "+",
            UnaryOperator::Negate => "-"
        })
    }

}

pub fn parse(mut tokens: vec_deque::VecDeque<Token>) -> Result<Ast, String> {
    match parse_expression(&mut tokens){
        Ok(_root) if !tokens.is_empty() => Err(format!("Trailing tokens: {}", tokens.iter().fold("".to_string(), |acc, element| { format!("{}{:?}, ", acc, element) }).strip_suffix(", ").unwrap())),
        Ok(root) => Ok(Ast { root }),
        Err(msg) => Err(format!("{msg}"))
    }
}

fn parse_expression(tokens: &mut vec_deque::VecDeque<Token>) -> Result<AstNode, String> {

    fn parse_expression_rest(tokens: &mut vec_deque::VecDeque<Token>, lhs: AstNode) -> Result<AstNode, String> {
        let operator: Option<BinaryOperator> = match tokens.front() {
            Some(token) => match token {
                Token::Operator { operator } => match operator {
                    Operator::Plus => Some(BinaryOperator::Add),
                    Operator::Minus => Some(BinaryOperator::Subtract),
                    _ => None
                }
                _ => None
            }
            None => None
        };
        match operator {
            None => Ok(lhs),
            Some(operator) => {
                tokens.pop_front().unwrap();
                let rhs: AstNode = parse_term(tokens)?;
                parse_expression_rest(tokens, AstNode::BinaryOperation {
                    operator,
                    left: Box::new(lhs),
                    right: Box::new(rhs)
                })
            }
        }
    }

    let lhs: AstNode = parse_term(tokens)?;
    parse_expression_rest(tokens, lhs)
}

fn parse_term(tokens: &mut vec_deque::VecDeque<Token>) -> Result<AstNode, String> {

    fn parse_term_rest(tokens: &mut vec_deque::VecDeque<Token>, lhs: AstNode) -> Result<AstNode, String> {
        let operator: Option<BinaryOperator> = match tokens.front() {
            Some(token) => match token {
                Token::Operator { operator } => match operator {
                    Operator::Times => Some(BinaryOperator::Multiply),
                    Operator::Divide => Some(BinaryOperator::Divide),
                    _ => None
                }
                _ => None
            }
            None => None
        };
        match operator {
            None => Ok(lhs),
            Some(operator) => {
                tokens.pop_front().unwrap();
                let rhs: AstNode = parse_factor(tokens)?;
                parse_term_rest(tokens, AstNode::BinaryOperation {
                    operator,
                    left: Box::new(lhs),
                    right: Box::new(rhs)
                })
            }
        }
    }

    let lhs: AstNode = parse_factor(tokens)?;
    parse_term_rest(tokens, lhs)
}


fn parse_factor(tokens: &mut vec_deque::VecDeque<Token>) -> Result<AstNode, String> {
    if let Some(token) = tokens.front() && let Token::Operator { operator } = token {
        match operator {
            Operator::Plus => {
                tokens.pop_front().unwrap();
                return Ok(AstNode::UnaryOperation {
                    operator: UnaryOperator::Plus,
                    value: Box::new(parse_factor(tokens)?)
                });
            },
            Operator::Minus => {
                tokens.pop_front().unwrap();
                return Ok(AstNode::UnaryOperation {
                    operator: UnaryOperator::Negate,
                    value: Box::new(parse_factor(tokens)?)
                });
            },
            _ => {}
        }
    }
    let lhs: AstNode = match tokens.pop_front() {
        Some(token) => match token {
            Token::Number { value } => {
                Ok(AstNode::Operand { value })
            }
            Token::LeftParenthesis => {
                let expression: AstNode = parse_expression(tokens)?;
                if let Some(token) = tokens.pop_front() && let Token::RightParenthesis = token {
                    Ok(expression)
                } else {
                    Err("Missing closing parenthesis".to_string())
                }
            }
            t => {
                Err(format!("Expected number or expression, found {t:?}"))
            }
        }
        None => {
            Err("Unterminated expression".to_string())
        }
    }?;
    if let Some(token) = tokens.front() && let Token::Operator { operator } = token && matches!(operator, Operator::Power) {
        tokens.pop_front().unwrap();
        let rhs: AstNode = parse_factor(tokens)?;
        Ok(AstNode::BinaryOperation {
            operator: BinaryOperator::Power,
            left: Box::new(lhs),
            right: Box::new(rhs)
        })
    } else {
        Ok(lhs)
    }
}