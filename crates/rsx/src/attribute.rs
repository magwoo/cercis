use crate::prelude::*;

pub struct ParseAttribute {
    pub name: String,
    pub value: Option<String>,
}

impl Parse for ParseAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        let _ = input.parse::<Token![:]>()?;
        let value = input.parse::<syn::LitStr>()?.value();
        let _ = input.parse::<Token![,]>()?;

        Ok(Self {
            name,
            value: Some(value),
        })
    }
}

impl ToTokens for ParseAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.name.as_str();

        match self.value.as_ref() {
            Some(value) => quote!(Attribute::new(#name).value(#value)),
            None => quote!(Attribute::new(#name)),
        }
        .to_tokens(tokens)
    }
}
