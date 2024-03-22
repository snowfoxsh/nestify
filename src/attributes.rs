use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{bracketed, LitInt, Meta, Token, token};
use syn::parse::{Parse, ParseStream};

/// ```
/// // In structs
/// struct MyStruct {
///     #[field_attribute]
///     field: ()
/// }
///
/// // In enums
/// enum MyEnum {
///     #[field_attribute]
///     MyNamed {
///         #[field_attribute]
///         field: ()
///     },
/// }
/// ```
pub enum FieldAttribute {
    Type(NestedAttribute),
    Item(Attribute),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Attribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
}

pub struct NestedAttribute {
    pub pound_token: Token![#],
    pub ident_token: Token![>],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub modifier: Option<AttributeModifier>,
}

#[derive(Clone)]
pub struct CompositeAttribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub modifier: Option<AttributeModifier>,
}

#[derive(Clone, Copy)]
pub enum AttributeModifier {
    Star(Token![*]),
    Slash(Token![/]),
    Minus(Token![-]),
}

pub trait ParseAttribute: Sized {
    fn parse_outer(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = vec![];
        while input.peek(Token![#]) {
            attrs.push(input.call(Self::parse_single_outer)?)
        }
        Ok(attrs)
    }
    fn parse_single_outer(input: ParseStream) -> syn::Result<Self>;
}


impl Parse for AttributeModifier {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![*]) {
            input.parse().map(Self::Star)
        } else if lookahead.peek(Token![/]) {
            input.parse().map(Self::Slash)
        } else if lookahead.peek(Token![-]) {
            input.parse().map(Self::Minus)
        } else {
            Err(lookahead.error())
        }
    }
}

impl ParseAttribute for Attribute {
    fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input),
            meta: content.parse()?,
        })
    }
}


impl ParseAttribute for FieldAttribute {
    fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![#]) && input.peek2(Token![>]) {
            Ok(Self::Type(input.call(NestedAttribute::parse_single_outer)?))
        } else {
            Ok(Self::Item(input.call(Attribute::parse_single_outer)?))
        }
    }
}


impl ParseAttribute for NestedAttribute {
    fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        let pound_token = input.parse()?;
        let ident_token = input.parse()?;
        let bracket_token = bracketed!(content in input);
        let meta = content.parse()?;

        let modifier =
        if  input.peek(Token![*]) ||
            input.peek(Token![/]) ||
            input.peek(Token![-]) ||
            input.peek(Token![+]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            pound_token,
            ident_token,
            bracket_token,
            meta,
            modifier,
        })
    }
}

impl ParseAttribute for CompositeAttribute {
    fn parse_single_outer(input: ParseStream) -> syn::Result<Self> {
        let content;
        let pound_token = input.parse()?;
        let bracket_token = bracketed!(content in input);
        let meta = content.parse()?;

        let modifier =
        if  input.peek(Token![*]) ||
            input.peek(Token![/]) ||
            input.peek(Token![-]) ||
            input.peek(Token![+]) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            pound_token,
            bracket_token,
            meta,
            modifier,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);

        self.bracket_token.surround(tokens, |meta_tokens| {
            self.meta.to_tokens(meta_tokens)
        })
    }
}

impl ToTokens for CompositeAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);

        self.bracket_token.surround(tokens, |meta_tokens| {
            self.meta.to_tokens(meta_tokens)
        })
    }
}

impl ToTokens for NestedAttribute {
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

impl From<NestedAttribute> for Attribute {
    fn from(nested: NestedAttribute) -> Self {
        Attribute {
            pound_token: nested.pound_token,
            bracket_token: nested.bracket_token,
            meta: nested.meta,
        }
    }
}

impl From<CompositeAttribute> for Attribute {
    fn from(composite: CompositeAttribute) -> Self {
        Attribute {
            pound_token: composite.pound_token,
            bracket_token: composite.bracket_token,
            meta: composite.meta,
        }
    }
}

impl PartialEq<NestedAttribute> for Attribute {
    fn eq(&self, other: &NestedAttribute) -> bool {
        self.meta == other.meta
    }
}

impl PartialEq<Attribute> for NestedAttribute {
    fn eq(&self, other: &Attribute) -> bool {
       self.meta == other.meta 
    }
}

impl PartialEq<CompositeAttribute> for Attribute {
    fn eq(&self, other: &CompositeAttribute) -> bool {
        self.meta == other.meta
    }
}

impl PartialEq<Attribute> for CompositeAttribute {
    fn eq(&self, other: &Attribute) -> bool {
       self.meta == other.meta 
    }
}


// note: it is preferred that >#[...] comes after #[...]