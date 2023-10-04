use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::token;
use syn::{bracketed, Meta, Token};
use syn::parse::ParseStream;


#[derive(Clone)]
pub struct RecAttribute {
    pound_token: Token![#],
    bracket_token: token::Bracket,
    meta: Meta,
    star_token: Option<Token![*]>
}

#[derive(Clone)]
pub struct NestAttribute {
    at_token: Token![@],
    bracket_token: token::Bracket,
    meta: Meta,
    star_token: Option<Token![*]>
}

impl RecAttribute {
    pub fn inheritable(attrs: Vec<Self>) -> Vec<Self> {
        attrs.into_iter().filter(|at| at.star_token.is_some()).collect()
    }

    pub fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![#]) {
            attrs.push(input.call(Self::single_parse_outer)?);
        }
        Ok(attrs)
    }

    fn single_parse_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            meta: content.parse()?,
            star_token: input.parse()?
        })
    }
}

impl NestAttribute {
    pub fn inheritable(attrs: Vec<Self>) -> Vec<Self> {
        attrs.into_iter().filter(|at| at.star_token.is_some()).collect()
    }

    pub fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![@]) {
            attrs.push(input.call(Self::single_parse_outer)?);
        }
        Ok(attrs)
    }

    fn single_parse_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            at_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            meta: content.parse()?,
            star_token: input.parse()?
        })
    }
}

impl ToTokens for RecAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);

        self.bracket_token.surround(tokens, |tokens| {
            self.meta.to_tokens(tokens);
        });
    }
}

impl ToTokens for NestAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote!(#));

        self.bracket_token.surround(tokens, |tokens| {
            self.meta.to_tokens(tokens);
        })
    }
}
