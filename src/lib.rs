use proc_macro::{TokenStream as TokenStream1};
use quote::quote;

#[proc_macro]
pub fn nest(input: TokenStream1) -> TokenStream1 {
    quote!(
        println!("todo")
    ).into()
}
