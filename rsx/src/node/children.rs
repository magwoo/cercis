use proc_macro2::TokenTree;

use crate::prelude::*;

pub struct Children {
    ident: syn::Ident,
}

impl Parse for Children {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.peek(syn::Ident) {
            true => Ok(Self {
                ident: input.parse::<syn::Ident>()?,
            }),
            false => Err(syn::Error::new_spanned(
                input.parse::<TokenTree>()?,
                "Expected ident for add children",
            )),
        }
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.ident;

        quote!(#name).to_tokens(tokens)
    }
}
