use proc_macro2::Span;
use std::str::FromStr;

use crate::prelude::*;

pub struct TextFmt {
    pub format: String,
    pub args: Vec<TokenStream2>,
    pub is_raw: bool,
}

impl Parse for TextFmt {
    fn parse(input: ParseStream) -> Result<Self> {
        let lit = input.parse::<syn::LitStr>()?;

        let mut args = Vec::new();
        let mut format = lit.value().to_string();

        if !format.contains('{') {
            return Err(syn::Error::new(Span::call_site(), "Missing fmt's"));
        }

        format = format.replace("{{", "[[").replace("}}", "]]");

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
        let is_raw = lit.to_token_stream().to_string().starts_with('r');

        Ok(Self {
            format,
            args,
            is_raw,
        })
    }
}

impl ToTokens for TextFmt {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let format = self.format.as_str();
        let args = self.args.as_slice();

        let raw_token = match self.is_raw {
            true => quote!(.raw()),
            false => quote!(),
        };

        quote!(
            .node(VNode::content(VContent::new(format!(#format, #(#args,)*))#raw_token))
        )
        .to_tokens(tokens)
    }
}
