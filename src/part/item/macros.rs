use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream {
    tokens: Vec<TokenTree>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenTree {
    Group(Group),
    Token(Token),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Group {
    stream: Box<TokenStream>,
    delimiter: Delimiter,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
    Parantheses,
    Square,
    Angle,
}

fn token_list() -> impl Parser<Token, Vec<Token>, Error = Simple<Token>> {
    take_until(one_of([
        op(OP_LPARA), op(OP_RPARA)
    ]))
}

pub fn token_stream() -> impl Parser<Token, TokenStream, Error = Simple<Token>> {

}