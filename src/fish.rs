use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Generics, Token};
use syn::parse::{Parse, ParseStream};


#[derive(Clone, Default)]
pub struct Fish {
    path_sep_token: Token![::],
    generics: Generics,
}

impl Parse for Fish {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if !input.peek(Token![::]) {
            return Ok(Self::default());
        }


        Ok(Self {
            path_sep_token: input.parse()?,
            generics: input.parse()?,
        })
    }
}

impl ToTokens for Fish {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens)
    }
}
