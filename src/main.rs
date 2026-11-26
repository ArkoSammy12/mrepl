mod lexer;
mod parser;

use parser::{UnaryOperator, BinaryOperator, Ast, AstNode};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    'repl: loop {
        let input: String = read_line()?;
        match input.as_str() {
            "exit" => {
                break 'repl;
            }
            _ => {
                match lexer::tokenize(&input) {
                    Err(msg) => println!("Tokenize error: {msg}!"),
                    Ok(tokens) => match parser::parse(tokens) {
                        Err(msg) => println!("Parse error: {msg}!"),
                        Ok(ast) => match evaluate(&ast) {
                            Err(msg) => println!("Cannot evaluate: {msg}!"),
                            Ok(value) => println!("{value}")
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn read_line() -> Result<String, std::io::Error> {
    print!(">");
    std::io::stdout().flush()?;
    let mut buff: String = String::from("");
    std::io::stdin().read_line(&mut buff)?;
    Ok(buff.trim().to_string())
}

fn evaluate(ast: &Ast) -> Result<f64, String> {
    evaluate_node(&ast.root)
}

fn evaluate_node(node: &AstNode) -> Result<f64, String> {
    Ok(match node {
        AstNode::Operand { value } => *value,
        AstNode::UnaryOperation { operator, value } => {
            let value = evaluate_node(value)?;
            match operator {
                UnaryOperator::Plus => value,
                UnaryOperator::Negate => -value
            }
        }
        AstNode::BinaryOperation { operator, left, right } => {
            let lhs = evaluate_node(left)?;
            let rhs = evaluate_node(right)?;
            match operator {
                BinaryOperator::Add => lhs + rhs,
                BinaryOperator::Subtract => lhs - rhs,
                BinaryOperator::Multiply => lhs * rhs,
                BinaryOperator::Divide => {
                    if rhs == 0.0f64 {
                        return Err("Cannot divide by zero".to_string())
                    } else {
                        lhs / rhs
                    }
                }
                BinaryOperator::Power => lhs.powf(rhs)
            }
        }
    })
}