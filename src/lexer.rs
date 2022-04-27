use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Float(f64),
    Symbol(String),
    Define,
    Begin,
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Float(n) => write!(f, "{}", n),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::Begin => write!(f, "begin"),
            Token::Define => write!(f, "define"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

pub struct TokenError {
    ch: char,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected character: {}", self.ch)
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, TokenError> {
    let program2 = program.replace("(", " ( ").replace(")", " ) ");
    let words = program2.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            "define" => tokens.push(Token::Define),
            "begin" => tokens.push(Token::Begin),
            "+" => tokens.push(Token::Plus),
            "-" => tokens.push(Token::Minus),
            "*" => tokens.push(Token::Multiply),
            "/" => tokens.push(Token::Divide),
            _ => {
                let mut chars = word.chars();
                let first_char = chars.next().unwrap();
                if first_char.is_digit(10) && word.contains(".") {
                    let float = word.parse::<f64>().unwrap();
                    tokens.push(Token::Float(float));
                } else if first_char.is_digit(10) {
                    let integer = word.parse::<i64>().unwrap();
                    tokens.push(Token::Integer(integer));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let tokens = tokenize("(+ 1 2)").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Plus,
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_area_of_a_circle() {
        let tokens =
            tokenize("(begin (define r 10)(define pi 3.14)(* pi (* r r)))").unwrap_or(vec![]);
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Begin,
                Token::LParen,
                Token::Define,
                Token::Symbol("r".to_string()),
                Token::Integer(10),
                Token::RParen,
                Token::LParen,
                Token::Define,
                Token::Symbol("pi".to_string()),
                Token::Float(3.14),
                Token::RParen,
                Token::LParen,
                Token::Multiply,
                Token::Symbol("pi".to_string()),
                Token::LParen,
                Token::Multiply,
                Token::Symbol("r".to_string()),
                Token::Symbol("r".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen
            ]
        );
    }
}
