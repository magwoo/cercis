use crate::prelude::*;

mod attribute;
mod element;
mod node;
mod prelude;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<ParseRoot>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}

struct ParseRoot(Vec<ParseNode>);

impl Parse for ParseRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse::<ParseNode>()?);
        }

        Ok(Self(nodes))
    }
}

impl ToTokens for ParseRoot {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let nodes = self.0.as_slice();

        quote!({
            use cercis_html::prelude::*;
            Component::new()#(.node(#nodes))*
        })
        .to_tokens(tokens)
    }
}
