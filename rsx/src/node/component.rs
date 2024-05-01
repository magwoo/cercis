use proc_macro2::TokenTree;

use crate::prelude::*;
use crate::BodyCall;

pub struct Component {
    name: syn::Ident,
    props: Vec<Attribute>,
    children: Option<BodyCall>,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let mut props = Vec::new();
        let mut children = None;

        let block;
        braced!(block in input);

        while block.peek(syn::Ident) && block.peek2(Token![:]) {
            props.push(block.parse::<Attribute>()?);

            if !block.is_empty() {
                block.parse::<Token![,]>()?;
            }
        }

        if !block.is_empty() {
            children = Some(block.parse::<BodyCall>()?);
        }

        Ok(Self {
            name,
            props,
            children,
        })
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let props = self.props.as_slice();
        let struct_name = syn::Ident::new((name.to_string() + "Props").as_str(), name.span());

        let children = match &self.children {
            Some(body) => quote!(.children(#body)),
            None => quote!(),
        };

        quote!(.node(VNode::Component(Component::new(#name, Box::new(#struct_name::builder()#(.#props)*#children.build())))))
            .to_tokens(tokens)
    }
}

struct Attribute {
    name: syn::Ident,
    value: Vec<TokenTree>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        input.parse::<Token![:]>()?;

        let mut value = Vec::new();
        while !input.is_empty() && !input.peek(Token![,]) {
            value.push(input.parse::<TokenTree>()?);
        }

        Ok(Self { name, value })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let value = &self.value;

        quote!(#name(#(#value)*)).to_tokens(tokens)
    }
}
