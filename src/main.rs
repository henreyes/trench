
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(String),
    Number(f64),
}
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            },
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            },
            ' ' | '\n' | '\t' => {
                chars.next();
            },
            _ => { 
                while let Some(&nc) = chars.peek() {
                    if nc == '(' || nc == ')' || nc.is_whitespace() {
                        break;
                    } else {
                        current_token.push(nc);
                        chars.next();
                    }
                }
                if let Ok(n) = current_token.parse::<f64>() {
                    tokens.push(Token::Number(n));
                } else {
                    tokens.push(Token::Symbol(current_token.clone()));
                }
                current_token.clear();
            }
        }
    }

    tokens
}

struct TokenVec(Vec<Token>);

impl fmt::Display for TokenVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, token) in self.0.iter().enumerate() {
            if index > 0 { write!(f, " ")?; } 
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

impl fmt::Display for Token {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        Token::OpenParen => write!(f, "("),
        Token::CloseParen => write!(f, ")"),
        Token::Symbol(s) => write!(f, "{}", s),
        Token::Number(n) => write!(f, "{}", n),
    }
}
}

fn main() {
    let input = "(cond
        ((> 5 10) 'greater)
        ((< 5 10) 'lesser)
        (t 'equal))";
    let tokens = tokenize(input);
    let token_vec = TokenVec(tokens); 
    println!("{}", token_vec);
}
