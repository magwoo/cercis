use element::Element;
use expr::IfExpr;
use fmt::TextFmt;
use forloop::ForLoop;
use text::Text;

use crate::prelude::*;

mod element;
mod expr;
mod fmt;
mod forloop;
mod text;

pub enum NodeTree {
    Text(Text),
    TextFmt(TextFmt),
    IfExpr(IfExpr),
    ForLoop(ForLoop),
    Element(Element),
}

impl Parse for NodeTree {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::Ident) {
            return Ok(Self::Element(Element::parse(input)?));
        }

        if input.fork().parse::<TextFmt>().is_ok() {
            return Ok(Self::TextFmt(TextFmt::parse(input)?));
        }

        if input.peek(Token![if]) {
            return Ok(Self::IfExpr(IfExpr::parse(input)?));
        }

        if input.peek(Token![for]) {
            return Ok(Self::ForLoop(ForLoop::parse(input)?));
        }

        Ok(Self::Text(Text::parse(input)?))
    }
}

impl ToTokens for NodeTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Text(text) => text.to_tokens(tokens),
            Self::TextFmt(fmt) => fmt.to_tokens(tokens),
            Self::Element(element) => element.to_tokens(tokens),
            Self::IfExpr(expr) => expr.to_tokens(tokens),
            Self::ForLoop(forloop) => forloop.to_tokens(tokens),
        }
    }
}
