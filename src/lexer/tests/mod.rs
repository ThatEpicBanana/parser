
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

            SELECTOR("@s".to_string()),
            POSITION("~ ~ ~".to_string()),

            STRING("string1".to_string()),
            STRING("string2".to_string()),

            INTEGER("1_024".to_string()),
            FLOAT("3.141_5".to_string()),

            IDENTIFIER("Raycaster".to_string()),
            KEYWORD(keyword::KW_MOD),

            UNK_OPERATOR("^-^".to_string()),
            OPERATOR(operator::OP_PLUS)
        ]
    )
}