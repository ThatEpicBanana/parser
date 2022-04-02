
// use crate::lexer::{self, keyword, operator};
use chumsky::{prelude::*};

// use super::Token::{self, *};

use crate::lexer::prelude::*;

fn test(src: &str, expected: Vec<Token>) -> Result<(), Vec<Simple<char>>> {
    let result = lexer::create().parse_recovery(src);

    match result.0 {
        Some(x) => {
            let result: Vec<Token> = x.into_iter().map(|x| x.0).collect();

            assert_eq!(result, expected);
        },
        None => {
            return Err(result.1);
        },
    }

    Ok(())
}

#[test]
fn general_test() -> Result<(), Vec<Simple<char>>> {
    test(include_str!("general_test.of"), vec![
        BOOLEAN(true),

        string("string1"),
        string("string2"),

        INTEGER(65535),
        float("3.1415"),

        id("Raycaster"),
        kw(KW_MOD),

        unk_op("#.#"),
        op(OP_PLUS)
    ])
}

#[test]
fn number_separation() -> Result<(), Vec<Simple<char>>> {
    test(include_str!("number_separation.of"), vec![
        INTEGER(1024), INTEGER(65535), INTEGER(10), INTEGER(1234), INTEGER(6969420),
        float("3.14"), float("3.1415")
    ])
}

#[test]
fn real_world() -> Result<(), Vec<Simple<char>>> {
    test(include_str!("real_world_test.of"), vec![
        kw(KW_PUB), kw(KW_FUNC), id("main"), op(OP_LPARA), op(OP_RPARA), op(OP_LCURLY),
            kw(KW_AS), op(OP_AT), id("a"), op(OP_COLON),
                id("clear"), op(OP_LPARA), string("tnt"), id("id"), op(OP_RPARA), op(OP_SEMI),
        op(OP_RCURLY)
    ])
}

#[test]
fn comments() -> Result<(), Vec<Simple<char>>> {
    test(include_str!("comments.of"), vec![
        id("token"), id("here"), 
        id("these"), id("should"), id("though"), 
    ])
}