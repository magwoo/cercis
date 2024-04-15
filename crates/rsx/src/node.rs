use crate::prelude::*;

pub enum ParseNode {
    Element(ParseElement),
    Text(String),
    TextFmt(String),
    IfExpr(ParseIfExpr),
}

impl Parse for ParseNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let mut attributes = Vec::new();
        let mut children = Vec::new();

        let block;
        braced!(block in input);

        while block.peek(syn::Ident) && block.peek2(Token![:]) {
            attributes.push(block.parse::<ParseAttribute>()?)
        }

        while !block.is_empty() {
            if block.peek(syn::LitStr) {
                let text = block.parse::<syn::LitStr>()?.value();
                match text.contains('{') {
                    true => children.push(Self::TextFmt(text)),
                    false => children.push(Self::Text(text)),
                }
            } else if block.peek(syn::Ident) {
                children.push(block.parse::<Self>()?)
            }
        }

        Ok(Self::Element(ParseElement {
            name: name.to_string(),
            attributes,
            children,
        }))
    }
}

impl ToTokens for ParseNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ParseNode::Element(element) => {
                let name = element.name.as_str();
                let attributes = element.attributes.as_slice();
                let children = element.children.as_slice();

                quote!(Node::Element(Element::new(#name)#(.attr(#attributes))*#(.child(#children))*))
            }
            ParseNode::Text(text) => quote!(Node::content(#text)),
            ParseNode::TextFmt(text) => quote!(Node::content(format!(#text))),
        }.to_tokens(tokens)
    }
}
