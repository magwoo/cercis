use crate::prelude::*;
use crate::NodeTree;

pub struct Element {
    name: String,
    children: Vec<NodeTree>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        let mut children = Vec::new();

        let block;
        braced!(block in input);

        while !block.is_empty() {
            children.push(block.parse::<NodeTree>()?);
        }

        Ok(Self { name, children })
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();
        let children = self.children.as_slice();

        quote!(
            .node(VNode::element(VElement::new(#name)#(#children)*))
        )
        .to_tokens(tokens)
    }
}
