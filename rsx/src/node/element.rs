use crate::prelude::*;
use crate::NodeTree;

pub struct Element {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<NodeTree>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        let mut attributes = Vec::new();
        let mut children = Vec::new();

        let block;
        braced!(block in input);

        while block.peek(syn::Ident) && block.peek2(Token![:]) {
            attributes.push(block.parse::<Attribute>()?);

            if !block.is_empty() {
                block.parse::<Token![,]>()?;
            }
        }

        while !block.is_empty() {
            children.push(block.parse::<NodeTree>()?);
        }

        Ok(Self {
            name,
            attributes,
            children,
        })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();
        let attributes = self.attributes.as_slice();
        let children = self.children.as_slice();

        quote!(
            .node(VNode::element(VElement::new(#name)#(#attributes)*#(#children)*))
        )
        .to_tokens(tokens)
    }
}

struct Attribute {
    name: String,
    value: Option<String>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        input.parse::<Token![:]>()?;
        let value = input.parse::<syn::LitStr>()?.value();

        Ok(Self {
            name,
            value: Some(value),
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();
        let value = self.value.as_ref();

        match value {
            Some(value) => quote!(.attr(Attribute::new(#name).value(#value))),
            None => quote!(.attr(Attribute::new(#name))),
        }
        .to_tokens(tokens)
    }
}
