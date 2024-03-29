use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(String),
    Number(f64),
    Quote,
    Nil,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::Quote => write!(f, "'"),
            Token::Nil => write!(f, "Nil"),
        }
    }
}

pub struct TokenVec(Vec<Token>);
impl fmt::Display for TokenVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, token) in self.0.iter().enumerate() {
            if index > 0 { write!(f, " ")?; } 
            write!(f, "{}", token)?;
        }
        Ok(())
    }
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
            '\'' => {
                tokens.push(Token::Quote); 
                chars.next();
            },
            ' ' | '\n' | '\t' => {
                chars.next(); 
            },
            _ if c.is_digit(10) || c == '.' => {
                while let Some(&nc) = chars.peek() {
                    if nc.is_digit(10) || nc == '.' {
                        current_token.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = current_token.parse::<f64>() {
                    tokens.push(Token::Number(n));
                }
                current_token.clear();
            },
            _ => {
                while let Some(&nc) = chars.peek() {
                    if nc == '(' || nc == ')' || nc.is_whitespace() || nc == '\'' {
                        break;
                    } else {
                        current_token.push(nc);
                        chars.next();
                    }
                }
                if current_token == "nil" {
                    tokens.push(Token::Nil);
                } else {
                    tokens.push(Token::Symbol(current_token.clone()));
                }
                current_token.clear();
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_quote() {
        let input = "'(1 2 3)";
        let expected = vec![
            Token::Quote,
            Token::OpenParen,
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::CloseParen,
        ];

        let result = tokenize(input);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_tokenize_defun() {
        let input = "(defun my_func (x) (+ x 3))";
        let expected = vec![
            Token::OpenParen,
            Token::Symbol("defun".to_string()),
            Token::Symbol("my_func".to_string()),
            Token::OpenParen,
            Token::Symbol("x".to_string()),
            Token::CloseParen,
            Token::OpenParen,
            Token::Symbol("+".to_string()),
            Token::Symbol("x".to_string()),
            Token::Number(3.0),
            Token::CloseParen,
            Token::CloseParen,
        ];
        let result = tokenize(input);
        assert_eq!(result, expected);
    }
}