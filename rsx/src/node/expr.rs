use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::TokenTree;

use crate::prelude::*;
use crate::BodyCall;

pub struct IfExpr {
    cond: Vec<TokenTree>,
    then_branch: BodyCall,
    else_if_branch: Option<Box<Self>>,
    else_branch: Option<BodyCall>,
}

impl Parse for IfExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![if]>()?;

        let mut cond = Vec::new();

        while !input
            .fork()
            .parse::<Group>()
            .is_ok_and(|g| g.delimiter() == Delimiter::Brace)
        {
            cond.push(input.parse::<TokenTree>()?);
        }

        let block;
        braced!(block in input);

        let then_branch = block.parse::<BodyCall>()?;

        let (else_branch, else_if_branch) = match input.peek(Token![else]) {
            true if input.peek2(Token![if]) => {
                input.parse::<Token![else]>()?;

                (None, Some(Box::new(input.parse::<Self>()?)))
            }
            true => {
                input.parse::<Token![else]>()?;
                let block;
                braced!(block in input);

                (Some(block.parse::<BodyCall>()?), None)
            }
            false => (None, None),
        };

        Ok(Self {
            cond,
            then_branch,
            else_if_branch,
            else_branch,
        })
    }
}

impl ToTokens for IfExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let cond = self.cond.as_slice();
        let then_branch = &self.then_branch;
        let else_if_branch = &self.else_if_branch;
        let else_branch = &self.else_branch;

        let else_if_tokens = match else_if_branch {
            Some(else_if_branch) => quote!(else #else_if_branch),
            None => quote!(),
        };

        let else_tokens = match else_branch {
            Some(then_branch) => quote!(else { #then_branch }),
            None if else_if_branch.is_some() => quote!(),
            None => quote!(else { VBody::new() }),
        };

        quote!({
            if #(#cond)* { #then_branch }
            #else_if_tokens
            #else_tokens
        })
        .to_tokens(tokens)
    }
}
