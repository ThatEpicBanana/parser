use crate::prelude::*;
use std::iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PathRoot {
    This,
    Basket,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PathPart {
    Super,
    Selff,
    Id(Ident)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Path {
    pub root: Option<PathRoot>,
    pub parts: Vec<PathPart>,
}

fn string_to_path_part(string: &str) -> PathPart {
    match string {
        "super" => PathPart::Super,
        "self" => PathPart::Selff,
        x => PathPart::Id(x.into()),
    }
}

impl From<&str> for Path {
    /// Converts a string into a path
    /// 
    /// # Panics
    /// 
    /// - If the string has more than one colon
    fn from(string: &str) -> Path {
        let mut list: Vec<_> = string.split(":").collect();

        // handle optional colon
        let list: Box<dyn Iterator<Item = &str>> = match list.len() {
            0 => panic!("String being converted into path is empty!"),
            1 => {
                Box::new(
                    list.pop()
                        .unwrap()
                        .split(".")
                )
            }, 
            2 => {
                Box::new(
                    iter::once(
                        list.pop() // index 0
                            .unwrap()
                    ).chain(
                        list.pop() // index 1
                            .unwrap()
                            .split(".")
                    )
                )
            },
            3.. => panic!("String being converted into path has more than one colon (:)!"),
        };

        let mut parts = vec![];

        // get root
        let root = match list.next() {
            Some("this") => Some(PathRoot::This),
            Some("basket") => Some(PathRoot::Basket),
            Some(x) => { parts.push(string_to_path_part(x)); None }
            _ => None
        };

        // convert parts to PathParts
        for part in list {
            parts.push(string_to_path_part(part));
        }

        // return
        Path { root, parts }
    }
}

fn path_part() -> impl Parser<Token, PathPart, Error = Simple<Token>> {
    just(KW_SUPER).to(PathPart::Super)
        .or(just(KW_SELF).to(PathPart::Selff))
        .or(ident().map(|idt| PathPart::Id(idt)))
}

pub fn path() -> impl Parser<Token, Path, Error = Simple<Token>> {
    // root
    just(KW_BASKET).to(PathRoot::Basket)
        .or(just(KW_THIS).to(PathRoot::This))
        .or_not()
    .then(
        // optional : then part
        just(OP_COLON)
            .ignore_then(path_part())
            .or_not()
        .chain( // then repeated . then part
            just(OP_DOT)
                .ignore_then(path_part())
                .repeated()
        )
    ).map(|(root, parts)| Path{root, parts}) // map to path
}