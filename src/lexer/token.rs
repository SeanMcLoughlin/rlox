use crate::error::LoxError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum TokenType {
    // Single-character tokens
    #[strum(serialize = "(")]
    LeftParen,
    #[strum(serialize = ")")]
    RightParen,
    #[strum(serialize = "{")]
    LeftBrace,
    #[strum(serialize = "}")]
    RightBrace,
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = ".")]
    Dot,
    #[strum(serialize = "-")]
    Minus,
    #[strum(serialize = "+")]
    Plus,
    #[strum(serialize = ";")]
    Semicolon,
    #[strum(serialize = "/")]
    Slash,
    #[strum(serialize = "*")]
    Star,

    // One or two character tokens
    #[strum(serialize = "!")]
    Bang,
    #[strum(serialize = "!=")]
    BangEqual,
    #[strum(serialize = "=")]
    Equal,
    #[strum(serialize = "==")]
    EqualEqual,
    #[strum(serialize = ">")]
    Greater,
    #[strum(serialize = ">=")]
    GreaterEqual,
    #[strum(serialize = "<")]
    Less,
    #[strum(serialize = "<=")]
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    #[strum(serialize = "and")]
    And,
    #[strum(serialize = "class")]
    Class,
    #[strum(serialize = "else")]
    Else,
    #[strum(serialize = "false")]
    False,
    #[strum(serialize = "fun")]
    Fun,
    #[strum(serialize = "for")]
    For,
    #[strum(serialize = "if")]
    If,
    #[strum(serialize = "nil")]
    Nil,
    #[strum(serialize = "or")]
    Or,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "super")]
    Super,
    #[strum(serialize = "this")]
    This,
    #[strum(serialize = "true")]
    True,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "while")]
    While,

    #[strum(serialize = "EOF")]
    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            type_: TokenType::Eof,
            lexeme: String::new(),
            line: 0,
        }
    }
}

impl Token {
    pub fn new(type_: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            type_,
            lexeme,
            line,
        }
    }

    pub fn build(lexeme: &str, line: usize) -> Result<Self, LoxError> {
        let type_ = match TokenType::from_str(lexeme) {
            Ok(type_) => type_,
            Err(_) => {
                return Err(LoxError::InvalidToken {
                    line,
                    token: lexeme.into(),
                })
            }
        };
        Ok(Token::new(type_, lexeme.to_string(), line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_token() {
        let mut exp = Token {
            type_: TokenType::RightBrace,
            lexeme: String::new(),
            line: 2,
        };
        let mut act = Token::new(TokenType::RightBrace, String::new(), 2);
        assert_eq!(exp, act);

        exp = Token {
            type_: TokenType::EqualEqual,
            lexeme: String::from("e"),
            line: 3,
        };
        act = Token::new(TokenType::EqualEqual, String::from("e"), 3);
        assert_eq!(exp, act);
    }

    #[test]
    fn build_token() {
        let mut exp: Token = Token {
            type_: TokenType::And,
            lexeme: String::from("and"),
            line: 3,
        };
        let mut act = Token::build("and", 3).unwrap();
        assert_eq!(exp, act);

        exp = Token {
            type_: TokenType::Or,
            lexeme: String::from("or"),
            line: 4,
        };
        act = Token::build("or", 4).unwrap();
        assert_eq!(exp, act);
    }

    #[test]
    fn build_invalid_token_throws_error() {
        let exp = Some(LoxError::InvalidToken {
            line: 5,
            token: String::from("trash_string"),
        });
        let act = Token::build("trash_string", 5).err();
        assert_eq!(exp, act);
    }
}
