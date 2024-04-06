use crate::prelude::*;

pub struct ParseAttribute {
    pub name: String,
    pub value: Option<String>,
}

impl Parse for ParseAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?.to_string();
        input.parse::<Token![:]>()?;
        let value = input.parse::<syn::Lit>()?;

        match input.parse::<Token![,]>() {
            Err(_) if !input.is_empty() => {
                return Err(syn::Error::new(
                    value.span(),
                    "Expected comma after attribute `,`",
                ))
            }
            _ => (),
        }

        let value = match value {
            syn::Lit::Str(str) => str.value(),
            syn::Lit::Int(num) => num.to_string(),
            syn::Lit::Float(num) => num.to_string(),
            syn::Lit::Bool(value) => value.value().to_string(),
            _ => {
                return Err(syn::Error::new(
                    value.span(),
                    "Unexpected attribute value type",
                ))
            }
        };

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
