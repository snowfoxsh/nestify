use proc_macro_error::abort;
use syn::parse::{Parse, ParseStream};
use syn::Type;
use crate::special_data::Special;



/// Can either be a normal type, or a type definition
pub enum SpecialType {
    Def(Special),
    Type(Type),
}

// todo idea: macro methods

impl Parse for SpecialType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.span();
        if let Ok(special) = input.parse::<Special>() {
            Ok(SpecialType::Def(special))
        } else if let Ok(ty) = input.parse::<Type>() {
            Ok(SpecialType::Type(ty))
        } else {
            // let message = format!("Expected a type, enum, structure or enum. start: {:?} | end: {:?}", span, end);
            let error = input.error("hello");
            // abort!(
            //     span,
            //     message;
            //     help = "Try something like i32 or struct Example {}"
            // ) // todo: better help message and try to fix span
            Err(error)
        }
    }
}
