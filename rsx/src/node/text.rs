use crate::prelude::*;

pub struct Text {
    text: String,
    is_raw: bool,
}

impl Parse for Text {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;

        let text = lit.value();
        let is_raw = lit.to_token_stream().to_string().starts_with('r');

        Ok(Self { text, is_raw })
    }
}

impl ToTokens for Text {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let text = self.text.as_str();
        let raw_token = match self.is_raw {
            true => quote!(.raw()),
            false => quote!(),
        };

        quote!(
            .node(VNode::content(VContent::new(#text)#raw_token))
        )
        .to_tokens(tokens)
    }
}
