// imports
use std::fmt;
use chumsky::{prelude::*};

use keyword::Keyword;
use operator::Operator;


// modules
pub mod reserved;
mod atom;
pub mod prelude;

#[cfg(test)]
mod tests;


// reexports
pub use Token::*;
pub use reserved::*;


// span type
type Span = std::ops::Range<usize>;


// - token type stuff -
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    // true or false
    BOOLEAN(bool),

    // "" or ''
    STRING(String),

    // numbers
    INTEGER(usize),
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
            BOOLEAN(x) => write!(f, "{}", x),
            STRING(x) => write!(f, "{}", x),
            INTEGER(x) => write!(f, "{}", x),
            FLOAT(x) => write!(f, "{}", x),
            IDENTIFIER(x) => write!(f, "{}", x),
            KEYWORD(x) => write!(f, "{}", x),
            UNK_OPERATOR(x) => write!(f, "{}", x),
            OPERATOR(x) => write!(f, "{}", x),
        }
    }
}


// - reserved stuff -
fn identifier() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident::<char, _>()
        .map(|string| {
            if let Some(identifier) = keyword::from_string(&string) {
                KEYWORD(identifier)
            } else {
                match string.as_str() {
                    "true" => BOOLEAN(true),
                    "false" => BOOLEAN(false),
                    _ => IDENTIFIER(string),
                }
            }
        })
}


// choose(operator).or(filter_operator())

fn operator() -> impl Parser<char, Token, Error = Simple<char>> {
    filter::<char, _, _>(|c| c.is_ascii_punctuation())
        .repeated().at_least(1).collect()
        .map(|string| {
            if let Some(operator) = operator::from_string(&string) {
                OPERATOR(operator)
            } else {
                UNK_OPERATOR(string)
            }
        })
}


/// Creates a lexer which outputs a vector of [`Token`]s connected to their `Span`s 
/// 
/// # Examples:
/// ```
/// use parser::lexer::prelude::*;
/// 
/// let result = lexer::create().parse("as `@e`: say(\"hi\")").unwrap();
/// let result: Vec<Token> = result.into_iter().map(|x| x.0).collect();
/// 
/// assert_eq!(result, vec![
///     kw(KW_AS), 
///     op(OP_BACKTICK), op(OP_AT), IDENTIFIER("e".to_string()), op(OP_BACKTICK),
///     op(OP_COLON),
///     IDENTIFIER("say".to_string()),
///     STRING("hi".to_string()),
/// ]);
/// ```
pub fn create() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let token = choice((
        atom::string(),
        atom::float(),
        atom::integer(),
        identifier(),
        operator(),
    )).recover_with(skip_then_retry_until([]));
    

    token
        .map_with_span(|token, span| (token, span))
        .padded()
        .repeated()
}