use crate::prelude::*;

pub struct Text(String);

impl Parse for Text {
    fn parse(input: ParseStream) -> Result<Self> {
        let text = input.parse::<syn::LitStr>()?.value();

        Ok(Self(text))
    }
}

impl ToTokens for Text {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let text = self.0.as_str();

        quote!(
            .node(Node::content(#text))
        )
        .to_tokens(tokens)
    }
}
