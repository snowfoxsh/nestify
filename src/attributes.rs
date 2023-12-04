use syn::{bracketed, Meta, Token, token};
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
        if let Ok(ty) = TypeApplication::single_parse_outer(input) {
            Ok(Self::Type(ty))
        } else if let Ok(it) = ItemAttribute::single_parse_outer(input) {
            Ok(Self::Item(it))
        } else {
            Err(input.error("Expected Attribute"))
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
