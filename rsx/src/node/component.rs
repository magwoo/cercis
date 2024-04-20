use crate::prelude::*;

pub struct Component {
    name: syn::Ident,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;

        let block;
        braced!(block in input);

        Ok(Self { name })
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;

        quote!(.node(VNode::Component(Component::new(#name)))).to_tokens(tokens)
    }
}
