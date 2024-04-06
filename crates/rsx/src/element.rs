use crate::prelude::*;

pub struct ParseElement {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<ParseNode>,
}
