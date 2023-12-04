use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input};
use crate::special_data::{Body, Special};

mod special_data;
mod attributes;
mod syn_misc;
mod ty;

// todo: add a warning to the macro shows rules for struct UsesSemi;

#[proc_macro]
#[proc_macro_error]
pub fn nest(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        abort_call_site!(
            "unexpected end of input, expected one of: `struct`, `enum`, `union`";
            help = "add a nested type to use the nest! macro"
        ) // todo: flesh out help message
    }

    let def = parse_macro_input!(input as Special);

    // unpack(def).into()
    quote!(struct Works {}).into()
}

// todo: add attribute support
fn unpack(def: Special) -> proc_macro2::TokenStream {
    let attrs = def.attrs;
    let visablity = def.vis;
    let ident = def.ident;
    let generics = def.generics;

    match def.body {
        Body::Enum(_) => {},
        Body::Struct(_) => {},
    }

    quote!(/* todo */)
}
