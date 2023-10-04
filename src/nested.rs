use syn::{Token, Visibility, Ident, Generics, Attribute, Type, braced};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma};
use crate::attrs::{RecAttribute};
use crate::fish::Fish;

#[derive(Clone)]
pub struct NestedStruct {
    pub(crate) attrs: Vec<RecAttribute>, // this can have * at end
    pub(crate) vis: Visibility,
    pub(crate) struct_token: Token![struct],
    pub(crate) name: Ident,
    pub(crate) gen: Generics,
    pub(crate) brace_token: Brace,
    pub(crate) fields: Punctuated<NestedField, Comma>
}

#[derive(Clone)]
pub struct NestedField {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: Visibility,
    pub(crate) ident: Ident,
    pub(crate) turbo: Fish,
    pub(crate) colon_token: Token![:],
    pub(crate) ty: NestedType,
}

#[derive(Clone)]
pub enum NestedType {
    Struct(NestedStruct),
    Type(Type)
}

impl Parse for NestedStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            attrs: input.call(RecAttribute::parse_outer)?,
            vis: input.parse()?,
            struct_token: input.parse()?,
            name: input.parse()?,
            gen: input.parse()?,
            brace_token: braced!(content in input),
            fields: Punctuated::<NestedField, Comma>::parse_terminated(&content)?
        })
    }
}

impl Parse for NestedField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            ident: input.parse()?,
            turbo: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?
        })
    }
}

impl Parse for NestedType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(rs) = input.parse::<NestedStruct>() {
            Ok(Self::Struct(rs))
        } else if let Ok(ty) = input.parse::<Type>() {
            Ok(Self::Type(ty))
        } else {
            // todo: make this error message better
            Err(input.error("Expected either a type, struct def, or enum def"))
        }
    }
}