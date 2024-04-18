use crate::prelude::*;

pub struct Component {
    name: String,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();

        Ok(Self { name })
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();

        quote!().to_tokens(tokens)
    }
}
