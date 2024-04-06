use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Result, Token};

use crate::attribute::Attribute;

mod attribute;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<BodyCall>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

struct BodyCall(Vec<BodyNode>);

impl Parse for BodyCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse::<BodyNode>()?);
        }

        Ok(Self(nodes))
    }
}

impl ToTokens for BodyCall {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let nodes = self.0.as_slice();

        quote!({
            use cercis_html::*;
            Component::new()#(.node(#nodes))*
        })
        .to_tokens(tokens)
    }
}

enum BodyNode {
    Element(BodyElement),
    Content(String),
}

impl Parse for BodyNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let mut children = Vec::new();

        let block;
        braced!(block in input);

        while !block.is_empty() {
            if block.peek(syn::LitStr) {
                let text = block.parse::<syn::LitStr>()?.value();

                children.push(Self::Content(text));
            } else if block.peek(syn::Ident) {
                children.push(block.parse::<Self>()?)
            }
        }

        Ok(Self::Element(BodyElement {
            name: name.to_string(),
            attributes: Vec::new(),
            children,
        }))
    }
}

impl ToTokens for BodyNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            BodyNode::Element(element) => {
                let name = element.name.as_str();
                let children = element.children.as_slice();

                quote!(Node::Element(Element::new(#name)#(.child(#children))*)).to_tokens(tokens)
            }
            BodyNode::Content(content) => quote!(Node::content(#content)).to_tokens(tokens),
        }
    }
}

struct BodyElement {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<BodyNode>,
}
