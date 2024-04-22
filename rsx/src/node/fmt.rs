use proc_macro2::Span;
use std::str::FromStr;

use crate::prelude::*;

pub struct TextFmt {
    pub format: String,
    pub args: Vec<TokenStream2>,
}

impl TextFmt {
    pub fn from_str(format: &str) -> Result<Self> {
        let mut args = Vec::new();
        let mut format = format.to_string();

        if !format.contains('{') {
            return Err(syn::Error::new(Span::call_site(), "Missing fmt's"));
        }

        let mut counter = 0;
        while let Some(pos1) = format.find('{') {
            format.replace_range(pos1..=pos1, "[");
            if let Some(pos2) = format.find('}') {
                let fmt = TokenStream2::from_str(&format[pos1 + 1..pos2])?;
                format.replace_range(pos2..=pos2, "]");
                format.replace_range(pos1 + 1..pos2, counter.to_string().as_str());
                args.push(fmt);
                counter += 1;
            }
        }

        format = format.replace('[', "{").replace(']', "}");

        Ok(Self { format, args })
    }
}

impl Parse for TextFmt {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;
        Self::from_str(lit.value().as_str())
    }
}

impl ToTokens for TextFmt {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let format = self.format.as_str();
        let args = self.args.as_slice();

        quote!(
            .node(VNode::content(format!(#format, #(#args,)*)))
        )
        .to_tokens(tokens)
    }
}
