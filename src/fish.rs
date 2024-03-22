use syn::parse::{Parse, ParseStream};
use syn::Turbofish;
use syn::{Generics, Token};

#[derive(Clone, Default)]
pub struct GenFish {
    path_sep_token: Token![::],
    generics: Generics,
}

impl Parse for GenFish {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GenFish {
            path_sep_token: input.parse()?,
            generics: input.parse()?,
        })
    }
}
