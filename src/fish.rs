use std::f32::consts::E;
use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Generics, Token};
use syn::spanned::Spanned;

#[derive(Clone, Default, Debug)]
pub struct GenFish {
    pub prefix: Token![||],
    pub generics: Generics,
}

impl GenFish {
    pub fn span(&self) -> Span {
        // will provide better span on a nightly compiler
        self.prefix.span().join(self.generics.span()).unwrap_or_else(|| self.prefix.span())
    }
}

impl Parse for GenFish {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fish = Self {
            prefix: input.parse()?,
            generics: input.parse()?,
        };
        
        if fish.generics.params.iter().count() == 0 {
            return Err(syn::Error::new(fish.span(), "FishHook should not have empty or no generics"));
        }
        Ok(fish)
    }
}

impl ToTokens for GenFish {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens);
    }
}
