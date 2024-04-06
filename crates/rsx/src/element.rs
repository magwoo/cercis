use crate::prelude::*;

pub struct ParseElement {
    pub name: String,
    pub attributes: Vec<ParseAttribute>,
    pub children: Vec<ParseNode>,
}
