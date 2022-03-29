mod keyword;
mod operator;

use std::fmt;

use chumsky::{prelude::*};
use keyword::Keyword;
use operator::Operator;


type Span = std::ops::Range<usize>;
fn concat(a: String, b: String) -> String { a + &b }

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

use Token::*;

// seperated into another function due to its size
fn string() -> impl Parser<char, Token, Error = Simple<char>> {
    //TODO: Multiline Strings
    let string_escape = just('\\').ignore_then(
            just('\\')
        .or(just('\''))
        .or(just('"'))
        .or(just('n').to('\n')) // newline
        .or(just('r').to('\r')) // carraige return
        .or(just('t').to('\t')) // tab
        .or(just('u').ignore_then( // stolen from json example
            filter(|c: &char| c.is_digit(16))
            .repeated().exactly(4)
            .collect::<String>()
            .validate(|digits, span, emit| {
                char::from_u32(u32::from_str_radix(&digits, 16).unwrap()) // convert digits to u32 and then char
                .unwrap_or_else(|| { // if converting to char failed, print error
                    emit(Simple::custom(span, "invalid unicode character"));
                    '\u{FFFD}' // unicode replacement character
                })
            })
        ))
    );


    let quote_string = just('\'') // start with ' or "
        .ignore_then(
            filter(|c: &char| *c != '\'') // followed by any characters that aren't a quote
            .or(string_escape) // while checking for escapes
        .repeated()) // repeat
        .then_ignore(just('\'')); // then ignore the ending quote

    let double_quote_string = just('"') // start with ' or "
        .ignore_then(
            filter(|c: &char| *c != '"') // followed by any characters that aren't a quote
            .or(string_escape) // while checking for escapes
        .repeated()) // repeat
        .then_ignore(just('"')); // then ignore the ending quote


    quote_string.or(double_quote_string)
        .collect::<String>() // collect chars to a string
        .map(STRING) // and convert to a token (as an enum is just a function)
}


fn delim_number() -> impl Parser<char, String, Error = Simple<char>> {
    let delim_number = text::int(10);

    delim_number
        .then( // base-10 delimited by '_'
            just('_').repeated()
            .ignore_then(delim_number)
            .repeated() 
        ).then_ignore(just('_').or_not()) // ignore trailing '_'
        .foldl(concat) // concatenate all strings
}

fn integer() -> impl Parser<char, Token, Error = Simple<char>> {
    delim_number()
        .try_map(|s, span| Ok(INTEGER(
            s.parse().map_err( // parse integer and return error on failiure
                |e| Simple::custom(span, format!("{}", e))
            )?
        ))) // convert to integer and save
}

fn float() -> impl Parser<char, Token, Error = Simple<char>> {
    delim_number()
        .then_ignore(just('.'))
        .then(delim_number())
        .try_map(|(whole, decimal), span| {
            let full = format!("{}.{}", whole, decimal);
            full.parse::<f64>().map_err( // check if float is valid
                |e| Simple::custom(span, format!("{}", e))
            )?; // return error if not
            Ok(FLOAT(full)) // if it is valid, return string
        })
}


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


pub fn create() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let token = choice((
        string(),
        float(),
        integer(),
        identifier(),
        operator(),
    )).recover_with(skip_then_retry_until([]));
    

    token
        .map_with_span(|token, span| (token, span))
        .padded()
        .repeated()
}



#[cfg(test)]
mod tests;