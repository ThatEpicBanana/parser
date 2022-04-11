use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub inner: bool,
    pub path: path::Path,
    pub value: Vec<Token>,
}

fn doc_comment(inn: bool) -> impl Parser<Token, Attribute, Error = Simple<Token>> {
    filter(move |tok: &Token| 
        matches!(tok, 
            DOC_COMMENT { com: _, inner } 
                if inner == &inn.clone()
        )
    ).map(move |tok| 
        Attribute{ 
            inner: inn, 
            path: "doc".into(), 
            value: vec![tok] 
        } 
    )
}

fn outer_attribute() -> impl Parser<Token, Attribute, Error = Simple<Token>> {
    just(OP_HASH)
        .ignore_then(path::path()
            .then(
                just(OP_EQUAL)
            )
        ).delimited_by(just(OP_LSQUARE), just(OP_RSQUARE))
}