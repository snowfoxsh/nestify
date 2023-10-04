mod nested;
mod fish;
mod attrs;

use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use syn::parse_macro_input;
use crate::attrs::RecAttribute;
use crate::nested::{NestedStruct, NestedType};

#[proc_macro]
pub fn nest(input: TokenStream1) -> TokenStream1 {
    let item = parse_macro_input!(input as NestedStruct);

    unpack(item, Vec::new()).into()
}

fn unpack(item: NestedStruct, inherited_attrs: Vec<RecAttribute>) -> TokenStream2 {
    let mut fields = Vec::new();
    let mut definitions = Vec::new();

    // set up inherited attributes
    let mut attrs_to_inherit = inherited_attrs.clone();
    attrs_to_inherit.append(&mut RecAttribute::inheritable(item.attrs.clone()));

    for field in item.fields {
        let attrs = field.attrs;
        let vis = field.vis;
        let ident = field.ident;
        let turbo = field.turbo;

        match field.ty {
            NestedType::Type(ty) => {
                // todo: add warning messages about turbofish syntax being used when it is not needed
                fields.push(quote!(#(#attrs)* #vis #ident : #ty #turbo));
            },
            NestedType::Struct(x) => {
                let ty = x.name.clone();
                fields.push(quote!(#(#attrs)* #vis #ident : #ty #turbo));

                let out = unpack(x, attrs_to_inherit.clone());
                definitions.push(out);
            },
        }
    }

    let attrs = item.attrs;
    let vis = item.vis;
    let name = item.name;
    let gen = item.gen;

    quote!(
        #(#inherited_attrs)* #(#attrs)*
        #vis struct #name #gen {
            #(#fields),*
        }

        #(#definitions)*
    )
}
