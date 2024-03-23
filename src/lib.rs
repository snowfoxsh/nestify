use crate::special_data::Special;
use crate::unpack::Unpack;
use crate::unpack_context::UnpackContext;
use proc_macro_error::{abort_call_site, proc_macro_error};
use syn::parse_macro_input;

pub(crate) mod attributes;
pub(crate) mod discriminant;
pub(crate) mod fish;
pub(crate) mod special_data;
#[cfg(test)]
mod tests;
pub(crate) mod ty;
pub(crate) mod unpack;
pub(crate) mod unpack_context;

// todo: add a warning to the macro shows rules for struct UsesSemi;
// todo: fix where clauses
// todo: fix issue where `struct { };` "unexpected `;`" error is not spanned correctly
// todo: add diagnostic warnings and possibly errors behind a feature flag for nightly users
// todo: use `quote_spanned!` when necessary
// todo: write more tests
// todo: add warning to put #>[meta] after #[meta]

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

    // unpack::unpack(def).into()
    def.unpack(UnpackContext::default(), Vec::default()).into()
    // quote!(struct Works {}).into()
}

/*
nest! {
    struct Parent<'p, P> {
        child::<'p, P> : struct Child<'c, C> {
            s: &'c str,
            f: C
        }
    }
}

nest! {
    struct Parent<T> {
        child::<T>
    }
}


struct P <T> {
    a::<T> : T
}
*/

/*
enum Hello {
    CLike = 3,
    Empty,
    Tuple(i32, struct Nested;),
    Named {
        ty: i32,
        nested: struct Nested;,
    }
}

 */
