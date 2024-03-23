use proc_macro2::{Span, TokenStream};
use quote::{ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Generics, Token};
use syn::spanned::Spanned;

#[derive(Clone, Default, Debug)]
pub struct FishHook {
    pub prefix: Token![||],
    pub generics: Generics,
}

impl FishHook {
    pub fn span(&self) -> Span {
        // will provide better span on a nightly compiler
        self.prefix.span().join(self.generics.span()).unwrap_or_else(|| self.prefix.span())
    }
}

impl Parse for FishHook {
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

impl ToTokens for FishHook {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens);
    }
}
