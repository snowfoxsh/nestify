use proc_macro_error::{abort_call_site, proc_macro_error};
use syn::parse_macro_input;
use crate::special_data::Special;
use crate::unpack::Unpack;

mod special_data;
mod attributes;
mod ty;
mod fish;
mod unpack;

// todo: add a warning to the macro shows rules for struct UsesSemi;



/*
// --- NOTES --- //
- A structure that is bare eg: struct MyStruct; should be expanded to struct MyStruct {}
- There should be no need for semi colons because of this
- the comma should act as a semi colon in the parse


// --- NOTES --- //
 */


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
    def.unpack().into()
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