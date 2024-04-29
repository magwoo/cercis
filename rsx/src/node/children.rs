use crate::prelude::*;

pub struct Children {
    ident: syn::Ident,
}

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse::<syn::Ident>()?,
        })
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.ident;

        quote!(.merge(#name.clone())).to_tokens(tokens)
    }
}
