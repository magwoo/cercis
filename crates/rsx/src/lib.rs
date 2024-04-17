use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Result;

use crate::node::NodeTree;

mod node;
mod prelude;

struct BodyCall(Vec<NodeTree>);

impl Parse for BodyCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(Self(nodes))
    }
}

impl ToTokens for BodyCall {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let nodes = self.0.as_slice();

        quote!({
            use cercis_html::*;
            Component::new()#(#nodes)*
        })
        .to_tokens(tokens)
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<BodyCall>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
