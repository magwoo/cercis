use proc_macro2::TokenTree;

use crate::prelude::*;
use crate::NodeTree;

use super::fmt::TextFmt;

mod single;

pub struct Element {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<NodeTree>,
    is_single: bool,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        let mut attributes = Vec::new();
        let mut children = Vec::new();

        let block;
        braced!(block in input);

        if name.as_str() == "doctype" {
            return Ok(Self {
                name: String::from("!DOCTYPE"),
                attributes: vec![Attribute {
                    name: String::from("html"),
                    value: None,
                    is_raw: false,
                }],
                children: Vec::new(),
                is_single: true,
            });
        }

        while (block.peek(syn::Ident) || block.peek(syn::LitStr)) && block.peek2(Token![:]) {
            attributes.push(block.parse::<Attribute>()?);

            if !block.is_empty() {
                block.parse::<Token![,]>()?;
            }
        }

        while !block.is_empty() {
            children.push(block.parse::<NodeTree>()?);
        }

        let is_single = single::SINGLE_TAGS.contains(&name.as_str());

        Ok(Self {
            name,
            attributes,
            children,
            is_single,
        })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();
        let attributes = self.attributes.as_slice();
        let children = self.children.as_slice();

        let single = match self.is_single {
            true => quote!(.single()),
            false => quote!(),
        };

        quote!(
            .node(VNode::element(VElement::new(#name)#(#attributes)*#(#children)*#single))
        )
        .to_tokens(tokens)
    }
}

struct Attribute {
    name: String,
    value: Option<Value>,
    is_raw: bool,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut is_raw = false;

        let name = match input.peek(syn::Ident) {
            true => input.parse::<syn::Ident>()?.to_string().replace('_', "-"),
            false => input.parse::<syn::LitStr>()?.value(),
        };
        #[allow(clippy::needless_late_init)]
        let value;
        input.parse::<Token![:]>()?;

        if input.peek(syn::Lit) {
            let fork = input.fork();
            let lit = input.parse::<syn::Lit>()?;

            value = Some(match lit {
                syn::Lit::Str(str) if str.value().contains('{') => {
                    let fmt = TextFmt::parse(&fork)?;
                    is_raw = fmt.is_raw;
                    Value::TextFmt(fmt)
                }
                syn::Lit::Str(str) => {
                    is_raw = str.to_token_stream().to_string().starts_with('r');
                    Value::Text(str.value())
                }
                syn::Lit::Int(num) => Value::Text(num.to_string()),
                syn::Lit::Float(num) => Value::Text(num.to_string()),
                syn::Lit::Bool(bool) => Value::Text(bool.value.to_string()),
                _ => return Err(syn::Error::new(lit.span(), "Unexpected value data type")),
            })
        } else {
            let token = input.parse::<TokenTree>()?;
            return Err(syn::Error::new(token.span(), "Unexpected value data type"));
        }

        Ok(Self {
            name,
            value,
            is_raw,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();
        let value = self.value.as_ref();

        let raw_token = match self.is_raw {
            true => quote!(.raw()),
            false => quote!(),
        };

        match value {
            Some(value) => quote!(.attr(VAttribute::new(#name).value(#value)#raw_token)),
            None => quote!(.attr(VAttribute::new(#name))),
        }
        .to_tokens(tokens)
    }
}

enum Value {
    Text(String),
    TextFmt(TextFmt),
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Text(text) => quote!(#text),
            Self::TextFmt(fmt) => {
                let format = fmt.format.as_str();
                let args = fmt.args.as_slice();

                quote!(format!(#format, #(#args,)*))
            }
        }
        .to_tokens(tokens)
    }
}
