use crate::special_data::Special;
use syn::parse::{Parse, ParseStream};
use syn::Type;

/// Can either be a normal type, or a type definition
pub enum SpecialType {
    Def(Special),
    Type(Type),
}

// idea: macro methods

impl Parse for SpecialType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(ty) = input.parse::<Type>() {
            Ok(SpecialType::Type(ty))
        } else {
            Ok(SpecialType::Def(input.parse::<Special>()?))
        }
    }
}
