use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{FnArg, Result};

struct Component(syn::ItemFn);

impl Parse for Component {
    fn parse(input: ParseStream) -> Result<Self> {
        let func = input.parse::<syn::ItemFn>()?;

        if let Some(async_token) = func.sig.asyncness {
            let message = "Component cannot be async";

            return Err(syn::Error::new(async_token.span, message));
        }

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

        let args = func.sig.inputs.iter();
        let prop_names = args.clone().map(|a| {
            let FnArg::Typed(a) = a else { unreachable!() };
            let syn::Pat::Ident(pt) = a.pat.as_ref() else {
                unreachable!()
            };
            pt.ident.clone()
        });
        let props = args.map(Prop::from).collect::<Vec<_>>();

        let body = func.block.as_ref();
        let output = &func.sig.output;
        let name = &func.sig.ident;
        let vis = &func.vis;
        let struct_name = syn::Ident::new((name.to_string() + "Props").as_str(), name.span());
        let generics = &func.sig.generics;

        quote!(
            #[derive(typed_builder::TypedBuilder)]
            #[builder(doc, crate_module_path=typed_builder)]
            struct #struct_name #generics {#(#props,)*}
            #[allow(non_snake_case)]
            #vis fn #name #generics(props: &dyn std::any::Any) #output {
                let #struct_name { #(#prop_names,)* } = props.downcast_ref().unwrap();
                #body
            }
        )
        .to_tokens(tokens)
    }
}

#[derive(Debug)]
struct Prop {
    prop: FnArg,
    is_opt: bool,
}

impl From<&FnArg> for Prop {
    fn from(value: &FnArg) -> Self {
        let mut is_opt = false;
        let value = value.clone();

        if let FnArg::Typed(pt) = &value {
            is_opt = pt.ty.to_token_stream().to_string().contains("Option <");
        }

        Self {
            prop: value,
            is_opt,
        }
    }
}

impl ToTokens for Prop {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attr = quote!(#[builder(default, setter(strip_option))]);
        let prop = &self.prop;

        if let FnArg::Typed(pt) = prop {
            if pt.ty.to_token_stream().to_string().as_str() == "Element" {
                quote!(#[builder(default = Element::default())] #prop).to_tokens(tokens);
                return;
            }
        }

        match self.is_opt {
            true => quote!(#attr #prop),
            false => quote!(#prop),
        }
        .to_tokens(tokens)
    }
}

#[proc_macro_attribute]
pub fn component(_: TokenStream, input: TokenStream) -> TokenStream {
    match syn::parse::<Component>(input.clone()) {
        Ok(component) => {
            let body = component.into_token_stream();
            body.into()
        }
        Err(err) => err.to_compile_error().into(),
    }
}
