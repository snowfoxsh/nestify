use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Generics, Token};

#[derive(Clone, Default, Debug)]
pub struct GenFish {
    path_sep_token: Token![::],
    generics: Generics,
}

impl Parse for GenFish {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path_sep_token: input.parse()?,
            generics: input.parse()?,
        })
    }
}

impl ToTokens for GenFish {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens)
    }
}
