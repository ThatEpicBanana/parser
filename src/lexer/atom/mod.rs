mod string;
pub use string::string;

mod number;
pub use number::integer;
pub use number::float;

use chumsky::prelude::*;
use crate::lexer::prelude::*;

// - reserved stuff -
pub fn identifier() -> impl Parser<char, Token, Error = Simple<char>> {
    text::ident::<char, _>()
        .map(|string| {
            if let Some(identifier) = super::reserved::keyword::from_string(&string) {
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


pub fn operator() -> impl Parser<char, Token, Error = Simple<char>> {
    choice((
        operator::any_cmp() // comparison operators ex: `==`
            .map(|op| OPERATOR(op, false)),
        operator::any_op() 
            .map(|op| OPERATOR(op, false)),
        operator::any_assign()
            .then(just('=').or_not())
            .map(|(op, assign)| OPERATOR(op, assign.is_some())),
        filter::<char, _, _>(|c| c.is_ascii_punctuation())
            .repeated().at_least(1).collect()
            .map(|op| UNK_OPERATOR(op))
    ))
}


pub fn comment() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    let single = just("//")
        .then(take_until(just('\n')))
        .padded().ignored();

    let multi = recursive(|multi| 
        just("/*")
            .then(take_until(
                just("*/").ignored()
                .or(multi)
            ))
            .padded().ignored()
    );

    single.or(multi)
}