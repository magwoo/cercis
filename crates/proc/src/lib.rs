use proc_macro2::token_stream;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

pub enum ParseError {
    ParseError,
}

pub trait Parse: Sized {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError>;
}

#[derive(Clone)]
pub struct ParseStream {
    stream: token_stream::IntoIter,
}

impl ParseStream {
    pub fn new(stream: impl Into<TokenStream>) -> Self {
        let stream = stream.into();
        let a = syn::parse2::<proc_macro2::TokenTree>(stream.clone()).unwrap();
        let stream = stream.into_iter();

        Self { stream }
    }

    pub fn is_empty(&self) -> bool {
        self.stream.clone().next().is_some()
    }

    pub fn pick(&mut self) -> Option<TokenTree> {
        self.stream.next()
    }

    pub fn peek<T: Parse>(&self) -> bool {
        T::parse(&mut self.clone()).is_ok()
    }

    pub fn fork(&self) -> Self {
        self.clone()
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }
}

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
}

pub struct Ident;

impl Parse for Ident {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if let Some(TokenTree::Ident(_)) = stream.pick() {
            Ok(Self)
        } else {
            Err(ParseError::ParseError)
        }
    }
}
