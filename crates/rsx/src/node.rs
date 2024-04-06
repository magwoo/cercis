use crate::prelude::*;

pub enum ParseNode {
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
