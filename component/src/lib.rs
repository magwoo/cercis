use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, FnArg, Result, Token};

struct Component(syn::ItemFn);

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let func = input.parse::<syn::ItemFn>()?;

        let name = &func.sig.ident;
        let first_char = name.to_string().chars().next().unwrap();

        if first_char.is_ascii_lowercase() {
            let message = "Expected first char at upper case";

            return Err(syn::Error::new(name.span(), message));
        }

        Ok(Self(func))
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let func = &self.0;

        let props = func.sig.inputs.iter().collect::<Vec<_>>();
        let prop_names = props.iter().map(|a| {
            let FnArg::Typed(a) = a else { unreachable!() };
            let syn::Pat::Ident(pt) = a.pat.as_ref() else {
                unreachable!()
            };
            pt.ident.clone()
        });
        let body = func.block.as_ref();
        let name = &func.sig.ident;
        let vis = &func.vis;
        let struct_name = syn::Ident::new((name.to_string() + "Props").as_str(), name.span());
        let generics = &func.sig.generics;

        quote!(
            struct #struct_name #generics {#(#props,)*}
            #[allow(non_snake_case)]
            #vis fn #name #generics(props: Box<dyn std::any::Any>) -> VBody {
                let #struct_name { #(#prop_names,)* } = props.downcast_ref().unwrap();
                #body
            }
        )
        .to_tokens(tokens)
    }
}

#[proc_macro_attribute]
pub fn component(_: TokenStream, input: TokenStream) -> TokenStream {
    match syn::parse::<Component>(input.clone()) {
        Ok(component) => component.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
