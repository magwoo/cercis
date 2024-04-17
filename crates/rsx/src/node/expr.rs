use proc_macro2::Group;
use proc_macro2::TokenTree;

use crate::prelude::*;
use crate::BodyCall;

pub struct IfExpr {
    cond: Vec<TokenTree>,
    then_branch: BodyCall,
}

impl Parse for IfExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![if]>()?;

        let mut cond = Vec::new();

        while input.fork().parse::<Group>().is_err() {
            cond.push(input.parse::<TokenTree>()?);
        }

        let block;
        braced!(block in input);

        let then_branch = block.parse::<BodyCall>()?;

        Ok(Self { cond, then_branch })
    }
}

impl ToTokens for IfExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let cond = self.cond.as_slice();
        let then_branch = &self.then_branch;

        quote!(.merge({
            let mut component = Component::new();
            if #(#cond)* {
                component = #then_branch
            }
            component
        }))
        .to_tokens(tokens)
    }
}
