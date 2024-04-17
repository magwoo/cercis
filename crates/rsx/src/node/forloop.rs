use proc_macro2::Group;
use proc_macro2::TokenTree;

use crate::prelude::*;
use crate::BodyCall;

pub struct ForLoop {
    cond: Vec<TokenTree>,
    body: BodyCall,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![for]>()?;

        let mut cond = Vec::new();

        while input.fork().parse::<Group>().is_err() {
            cond.push(input.parse::<TokenTree>()?);
        }

        let block;
        braced!(block in input);

        let body = block.parse::<BodyCall>()?;

        Ok(Self { cond, body })
    }
}

impl ToTokens for ForLoop {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let cond = self.cond.as_slice();
        let body = &self.body;

        quote!(.merge({
            let mut component = Component::new();
            for #(#cond)* { component.insert(#body) }
            component
        }))
        .to_tokens(tokens)
    }
}
