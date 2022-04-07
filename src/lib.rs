pub mod lexer;

use chumsky::{prelude::*, Stream, chain::Chain};
// use lexer::prelude::*;

type Span = std::ops::Range<usize>;

mod part;
mod prelude;

use prelude::*;

fn create() -> impl Parser<Token, Vec<Item>, Error = Simple<Token>> {
    just(id("temp")).to(Item::Error).repeated()
}

pub struct ParserOutput {
    out: Option<Vec<Item>>,
    lexer_errors: Vec<Simple<char>>,
    parse_errors: Vec<Simple<Token>>,
}

fn parse<'a, Iter, S>(input: S) -> ParserOutput 
where 
    Iter: Iterator<Item = (char, Span)> + 'a,
    S: Into<Stream<'a, char, Span, Iter>>,
{
    // length required for stream for some reason, probably for errors
    let len = input.len();

    // try lexing
    let (out, lexer_errors) = 
        lexer::create().parse_recovery(input);

    // check output of lexer
    let (out, parse_errors) = if let Some(out) = out {
        // if lexer succeeds, try parsing
        crate::create().parse_recovery(Stream::from_iter(len..len + 1, out.into_iter()))
    } else { 
        // if lexer fails, output lexer errors
        return ParserOutput{out: None, lexer_errors, parse_errors: Vec::new()};
    };

    // final output
    ParserOutput{out, lexer_errors, parse_errors}
}



#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
}
