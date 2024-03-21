use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{bracketed, Error, LitInt, Meta, Token, token};
use syn::parse::{Parse, ParseStream};

pub enum FieldAttribute {
    Type(TypeApplication),
    Item(ItemAttribute),
}

pub struct ItemAttribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub modifier: Option<AttributeModifier>,
}

pub struct TypeApplication {
    pub pound_token: Token![#],
    pub ident_token: Token![>],
    pub bracket_token: token::Bracket,
    pub meta: Meta,
    pub modifier: Option<AttributeModifier>,
}

pub enum AttributeModifier {
    Star (Token![*]),
    Slash (Token![/]),
    Minus (Token![-]),
    Plus {
        plus_token: Token![+],
        // note about depth, we should try to throw a warning if the depth value
        // doesn't reach a leaf
        // ie: it is too large
        depth: usize
    },
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
        } else if lookahead.peek(Token![+]) {
            Ok(Self::Plus {
                plus_token: input.parse()?,
                depth: input.parse::<LitInt>()?.base10_parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
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
            Ok(Self::Type(input.call(TypeApplication::single_parse_outer)?))
        } else {
            Ok(Self::Item(input.call(ItemAttribute::parse_single_outer)?))
        }
    }
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
        let pound_token = input.parse()?;
        let ident_token  = input.parse()?;
        let bracket_token = bracketed!(content in input);
        let meta = content.parse()?;
        
        let modifier = if  
            input.peek(Token![*]) ||
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
        let pound_token = input.parse()?;
        let bracket_token = bracketed!(content in input);
        let meta = content.parse()?;

        let modifier = if
            input.peek(Token![*]) ||
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
