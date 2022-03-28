mod keyword;
mod operator;

use std::fmt;

use chumsky::{prelude::*};
use keyword::Keyword;
use operator::Operator;

type Span = std::ops::Range<usize>;

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    // true or false
    BOOLEAN(bool),

    // Special Minecraft literals denoted by ``
    SELECTOR(String),
    POSITION(String),

    // "" or ''
    STRING(String),

    // numbers
    INTEGER(String),
    FLOAT(String),

    // identifier / keywords
    IDENTIFIER(String),
    KEYWORD(Keyword),

    // operators
    UNK_OPERATOR(String),
    OPERATOR(Operator),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // i don't know if there's a better way to do this
        match self {
            Token::BOOLEAN(x) => write!(f, "{}", x),
            Token::SELECTOR(x) => write!(f, "{}", x),
            Token::POSITION(x) => write!(f, "{}", x),
            Token::STRING(x) => write!(f, "{}", x),
            Token::INTEGER(x) => write!(f, "{}", x),
            Token::FLOAT(x) => write!(f, "{}", x),
            Token::IDENTIFIER(x) => write!(f, "{}", x),
            Token::KEYWORD(x) => write!(f, "{}", x),
            Token::UNK_OPERATOR(x) => write!(f, "{}", x),
            Token::OPERATOR(x) => write!(f, "{}", x),
        }
    }
}



pub fn create() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    
}

#[cfg(test)]
mod tests;