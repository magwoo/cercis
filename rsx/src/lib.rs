use crate::node::NodeTree;
use crate::prelude::*;

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

        let a = quote!({
            use cercis_html::prelude::*;
            VBody::new()#(#nodes)*
        });

        eprintln!("a: {}", a);

        a.to_tokens(tokens)
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<BodyCall>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
