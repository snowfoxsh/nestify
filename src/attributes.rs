use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, bracketed, Meta, Token, token};
use syn::parse::ParseStream;

pub struct ItemAttribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub star_token: Option<Token![*]>,
}

pub enum FieldAttribute {
    Type(TypeApplication),
    Item(ItemAttribute),
}

impl FieldAttribute {
    pub fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![#]) {
            attrs.push(input.call(Self::parse_single_outer)?)
        }
        Ok(attrs)
    }

    pub fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![#]) && input.peek2(Token![>]) {
            let content;
            Ok(Self::Type(TypeApplication {
                pound_token: input.parse()?,
                indent_token: input.parse()?,
                bracket_token: bracketed!(content in input),
                meta: content.parse()?,
                star_token: input.parse()?,
            }))
        } else if input.peek(Token![#]) {
            Ok(Self::Item(input.call(ItemAttribute::parse_single_outer)?))
        } else {
            // todo make this spanned
            // panic!("expected an attribute")
            Err(input.error("Expected an attribute"))
        }
    }
}


pub struct TypeApplication {
    pub pound_token: Token![#],
    pub indent_token: Token![>],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub star_token: Option<Token![*]>,
}

impl TypeApplication {
    fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![#]) && input.peek2(Token![>]) {
            attrs.push(input.call(Self::single_parse_outer)?);
        }
        Ok(attrs)
    }

    fn single_parse_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            pound_token: input.parse()?,
            indent_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            meta: content.parse()?,
            star_token: input.parse()?
        })
    }
}

impl ItemAttribute {
    pub(crate) fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![#]) && !input.peek2(Token![>]) {
            attrs.push(input.call(Self::parse_single_outer)?);
        }
        Ok(attrs)
    }

    fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            meta: content.parse()?,
            star_token: input.parse()?
        })
    }
}

impl ToTokens for ItemAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);

        self.bracket_token.surround(tokens, |meta_tokens| {
            self.meta.to_tokens(meta_tokens)
        })
    }
}

impl ToTokens for TypeApplication {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);

        self.bracket_token.surround(tokens, |meta_tokens| {
            self.meta.to_tokens(meta_tokens)
        })
    }
}

impl ToTokens for FieldAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldAttribute::Type(a) => a.to_tokens(tokens),
            FieldAttribute::Item(a) => a.to_tokens(tokens),
        }
    }
}

// note: it is preferred that >#[...] comes after #[...]

// nest! {
//     struct Hello2 {
//         >#[field_attribute]
//         >#[field_attribute]*
//         a: #[data_attribute] struct Another {
//         }
//     }
//
//     enum Hello2 {
//         >#[applied_field_attribute]
//         #[field_attribute]
//         Hello {
//             #[field_attribute]
//         }
//     }
// }
