#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    DefPub,
    Identifier(String),
    LeftParen,
    RightParen,
    Colon,
    Comma,
    IntType,
    Return,
    Plus,
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "defpub" => tokens.push(Token::DefPub),
                    "int" => tokens.push(Token::IntType),
                    "return" => tokens.push(Token::Return),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => unimplemented!("Error handling for unexpected characters"),
        }
    }

    tokens.push(Token::EOF);
    tokens
}
