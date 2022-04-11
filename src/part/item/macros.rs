use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenStream {
    trees: Vec<TokenTree>,
}

impl TokenStream {
    pub fn empty() -> TokenStream {
        TokenStream { trees: Vec::new() }
    }
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
    Curly,
    Parantheses,
    Square,
    Angle,
}

pub fn token_stream() -> impl Parser<Token, TokenStream, Error = Simple<Token>> {
    use Delimiter::*;

    recursive(|token_stream| 
        ( // token tree
            ( // group
                    // curly brackets
                token_stream.clone().delimited_by(just(OP_LCURLY), just(OP_RCURLY))
                    .recover_with(nested_delimiters( // damn recovery is thicc
                        OP_LCURLY, OP_RCURLY, 
                        [(OP_LPARA, OP_RPARA), (OP_LSQUARE, OP_RSQUARE), (OP_LANGLE, OP_RANGLE)],
                        |_| TokenStream::empty()
                    )).map(|stream| Group{ stream: Box::new(stream), delimiter: Curly })
                .or( // paranthesees
                    token_stream.clone().delimited_by(just(OP_LPARA), just(OP_RPARA))
                        .recover_with(nested_delimiters(
                            OP_LPARA, OP_RPARA, 
                            [(OP_LCURLY, OP_RCURLY), (OP_LSQUARE, OP_RSQUARE), (OP_LANGLE, OP_RANGLE)],
                            |_| TokenStream::empty()
                        )).map(|stream| Group{ stream: Box::new(stream), delimiter: Parantheses })
                ).or( // square brackets
                    token_stream.clone().delimited_by(just(OP_LSQUARE), just(OP_RSQUARE))
                        .recover_with(nested_delimiters(
                            OP_LSQUARE, OP_RSQUARE, 
                            [(OP_LCURLY, OP_RCURLY), (OP_LPARA, OP_RPARA), (OP_LANGLE, OP_RANGLE)],
                            |_| TokenStream::empty()
                        )).map(|stream| Group{ stream: Box::new(stream), delimiter: Square })
                ).or( // angle brackets
                    token_stream.clone().delimited_by(just(OP_LANGLE), just(OP_RANGLE))
                        .recover_with(nested_delimiters(
                            OP_LANGLE, OP_RANGLE, 
                            [(OP_LCURLY, OP_RCURLY), (OP_LPARA, OP_RPARA), (OP_LSQUARE, OP_RSQUARE)],
                            |_| TokenStream::empty()
                        )).map(|stream| Group{ stream: Box::new(stream), delimiter: Angle })
                )
            ).map(|group| TokenTree::Group(group))
            .or(any().map(|tok| TokenTree::Token(tok)))
        ).repeated()
         .map(|trees| TokenStream{trees})
    )
}