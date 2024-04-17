use crate::prelude::*;

pub struct TextFmt(String);

impl Parse for TextFmt {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;
        let text = lit.value();

        if !text.contains('{') {
            return Err(syn::Error::new(lit.span(), "Missing fmt's"));
        }

        Ok(Self(text))
    }
}

impl ToTokens for TextFmt {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let text = self.0.as_str();

        quote!(
            .node(Node::content(format!(#text)))
        )
        .to_tokens(tokens)
    }
}
