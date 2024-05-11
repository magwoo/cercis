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

        quote!({
            use ::cercis::html::*;
            VBody::new()#(#nodes)*
        })
        .to_tokens(tokens)
    }
}

/// Macro ```rsx!``` write HTML templates in Rust code like jsx
///
/// > Return VBody struct that can render to string by ```.render()``` method
///
/// # Examples
///
/// ## Formatting
///
/// ```
/// use cercis::prelude::*;
///
/// let text = "Hello World!";
/// rsx!(h1 { "{text}" });
/// ```
///
/// ## Nested
///
/// ```
/// use cercis::prelude::*;
///
/// let text = "Hello World!";
/// rsx!(div {
///   p { "{text}" }
///   span { "Lorem ipsum" }
/// });
/// ```
///
/// ## Attributes
///
/// ```
/// use cercis::prelude::*;
///
/// let container_name = "main";
/// rsx!(div {
///   class: "container-{container_name}",
///   
///   p { "Lorem ipsum" }
/// });
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    match syn::parse::<BodyCall>(input) {
        Ok(body) => body.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
