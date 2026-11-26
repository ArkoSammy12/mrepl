use std::collections::vec_deque;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Token {
    Number {
        value: f64
    },
    Operator {
        operator: Operator
    },
    LeftParenthesis,
    RightParenthesis
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Power
}

impl Display for Operator {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Times => "*",
            Operator::Divide => "/",
            Operator::Power => "^"
        })
    }

}

pub fn tokenize(line: &str) -> Result<vec_deque::VecDeque<Token>, String> {
    let mut chars: vec_deque::VecDeque<char> = line.chars().into_iter().collect();
    let mut tokens: vec_deque::VecDeque<Token> = vec_deque::VecDeque::new();
    while !chars.is_empty() {
        let c: char = chars.pop_front().unwrap();
        match c {
            '0'..='9' => {
                let mut num: String = String::from("");
                num.push(c);
                let mut dot: bool = false;
                'tok: while let Some(&num_char) = chars.front() {
                    match num_char {
                        digit if digit.is_digit(10) => {
                            chars.pop_front().unwrap();
                            num.push(digit)
                        }
                        '.' => {
                            if dot {
                                break 'tok;
                            } else {
                                dot = true;
                                chars.pop_front().unwrap();
                                num.push('.')
                            }
                        }
                        _ => {
                            break 'tok;
                        }
                    }
                }
                tokens.push_back(Token::Number {
                    value: num.parse::<f64>().unwrap()
                });
            }
            '-' => {
                tokens.push_back(Token::Operator {
                    operator: Operator::Minus
                })
            }
            '+' => {
                tokens.push_back(Token::Operator {
                    operator: Operator::Plus
                })
            }
            '*' => {
                tokens.push_back(Token::Operator {
                    operator: Operator::Times
                })
            }
            '/' => {
                tokens.push_back(Token::Operator {
                    operator: Operator::Divide
                })
            }
            '^' => {
                tokens.push_back(Token::Operator {
                    operator: Operator::Power
                })
            }
            '(' => {
                tokens.push_back(Token::LeftParenthesis)
            }
            ')' => {
                tokens.push_back(Token::RightParenthesis)
            }
            c if c.is_whitespace() => {}
            _ => {
                return Err(format!("Invalid token: {c}"))
            }
        }
    }
    Ok(tokens)
}