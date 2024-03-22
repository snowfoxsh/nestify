use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, Token};

pub struct Discriminant {
    pub eq_token: Token![=],
    pub expr: Expr,
}

impl ToTokens for Discriminant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.eq_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
