use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, Result, Token};

mod attribute;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<ParseRoot>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

struct ParseRoot(Vec<ParseNode>);

impl Parse for ParseRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse::<ParseNode>()?);
        }

        Ok(Self(nodes))
    }
}

impl ToTokens for ParseRoot {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let nodes = self.0.as_slice();

        quote!({
            use cercis_html::*;
            Component::new()#(.node(#nodes))*
        })
        .to_tokens(tokens)
    }
}

enum ParseNode {
    Element(ParseElement),
    Content(String),
}

impl Parse for ParseNode {
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

        Ok(Self::Element(ParseElement {
            name: name.to_string(),
            attributes: Vec::new(),
            children,
        }))
    }
}

impl ToTokens for ParseNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ParseNode::Element(element) => {
                let name = element.name.as_str();
                let children = element.children.as_slice();

                quote!(Node::Element(Element::new(#name)#(.child(#children))*)).to_tokens(tokens)
            }
            ParseNode::Content(content) => quote!(Node::content(#content)).to_tokens(tokens),
        }
    }
}

struct ParseElement {
    pub name: String,
    pub attributes: Vec<attribute::Attribute>,
    pub children: Vec<ParseNode>,
}
