use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Item {
    InnerAttribute(Opt<attribute::Attribute>),
    Error,
}

pub fn item() -> impl Parser<Token, Item, Error = Simple<Token>> {
    attribute::inner_attribute().map(Item::InnerAttribute)
}