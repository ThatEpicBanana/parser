use chumsky::prelude::*;
use crate::lexer::prelude::*;

//TODO: doc comments
pub fn comment() -> impl Parser<char, (), Error = Simple<char>> + Clone {
    let single = just("//")
        .then(none_of("/!"))
        .then(take_until(
            just("\r\n")
            .or(just("\n"))
        )).padded();

    let multi = just("/*").then(
        none_of("*!").ignored()
            .or(just("**").ignored())
        .then(recursive(|multi|
            take_until( // go until:
                just("/*") // another 
                    .then(multi.clone()) // account for the other
                    .then(multi) // account for the current one
                    .ignored()
                .or(just("*/") // or end
                    .ignored()
                ) 
            )
        ))
    );

    choice((
        just("/**/").ignored(),
        just("/***/").ignored(),
    )).or(choice((
        single.ignored(),
        multi.ignored(),
    ))).padded()
}

pub fn doc_comment() -> impl Parser<char, Token, Error = Simple<char>> {
    let single = just("//").ignored()
        .then( // check if inner or not
                just("!").to(true)
            .or(just("/").to(false))
        ).then( // go until end and collect into string
            take_until(
                just("\r\n")
                .or(just("\n"))
            ).map(|(vec, _)| vec) // reject newline
            .collect()
        ).map(|((_, inner), com)| DOC_COMMENT{ com, inner }); 

    single
}