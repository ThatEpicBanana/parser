
use crate::lexer::{self, keyword, operator};
use chumsky::{prelude::*};

use super::Token::{self, *};

#[test]
fn general_test() {
    let src = include_str!("general_test.of");

    let result = lexer::create().parse(src).unwrap();

    let result: Vec<Token> = result.into_iter().map(|x| x.0).collect();

    assert_eq!(
        result,
        vec![
            BOOLEAN(true),

            STRING("string1".to_string()),
            STRING("string2".to_string()),

            INTEGER(65535),
            FLOAT("3.1415".to_string()),

            IDENTIFIER("Raycaster".to_string()),
            KEYWORD(keyword::KW_MOD),

            UNK_OPERATOR("^-^".to_string()),
            OPERATOR(operator::OP_PLUS)
        ]
    )
}

#[test]
fn number_separation() -> Result<(), Vec<Simple<char>>> {
    let src = include_str!("number_separation.of");

    let result = lexer::create().parse_recovery(src);

    match result.0 {
        Some(x) => {
            let result: Vec<Token> = x.into_iter().map(|x| x.0).collect();

            assert_eq!(
                result,
                vec![
                    INTEGER(1024), INTEGER(65535), INTEGER(10), INTEGER(1234), INTEGER(6969420),
                    FLOAT("3.14".to_string()), FLOAT("3.1415".to_string())
                ]
            );
        },
        None => {
            return Err(result.1);
        },
    }

    Ok(())
}