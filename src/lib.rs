use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input};
use syn::Pat::Paren;
use crate::special_data::{Body, BodyStruct, Special, SpecialFields};
use crate::ty::SpecialType;

mod special_data;
mod attributes;
mod syn_misc;
mod ty;
mod fish;

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

    unpack(def).into()
    // quote!(struct Works {}).into()
}


// todo: add attribute support
fn unpack(def: Special) -> proc_macro2::TokenStream {
    let attrs = def.attrs;
    let visablity = def.vis;
    let ident = def.ident; // the definition name/type
    let generics = def.generics;

    // based on the type of the Special type [struct | enum | union?]
    // then determine the expansion
    match def.body {
        Body::Struct(body) => {

            // todo: if struct (;) token then advanced warning
            match body.fields {
                SpecialFields::Named(named) => {
                    // fields buffer load each
                    let mut fields = vec![];
                    let mut definitions = vec![];

                    // iterate through the fields
                    for field in named.named {
                        let attrs = field.attrs;
                        let vis = field.vis;
                        // unused field mutability see syn doc for FieldMutability
                        let _mutability = field.mutability;
                        // this is a named type, so there should always be an ident
                        // if there is no ident then there should be a parsing bug
                        let ident = field.ident.expect("ident missing. internal error?");

                        // branch off the type depending on if leaf is reached
                        match field.ty {
                            // leaf node aka non-special type => dont recurse
                            SpecialType::Type(ty) => {
                                let field_tokens = quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                );

                                fields.push(field_tokens);
                            }
                            // recuse down the parse stack
                            SpecialType::Def(special) => {
                                // trust that ty will be a defintion step
                                let ty = &special.ident; // dont move so no clone!
                                let field_tokens = quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                );
                                fields.push(field_tokens);

                                // unpack the definition of the type
                                // then add it to the definition buffer
                                // this could be one or more definition
                                // we don't care
                                let definition = unpack(special);
                                definitions.push(definition);
                            }
                        }
                    }



                    // define our current ctx struct
                    // - define attributes
                    // - define ident and specify generics
                    // - insert our previous definitions behind the struct
                    quote!(
                        // todo attrs
                        struct #ident #generics {
                            #(#fields)* ,
                        }

                        #(#definitions)*
                    )
                }
                // unpack a tuple struct or tuple variant
                // todo [change] remove the semicolon
                SpecialFields::Unnamed(unnamed) => {
                    // for field in unnamed.unnamed {
                    //
                    // }

                    quote!();
                    todo!()
                }
                SpecialFields::Unit => {
                    quote!();
                    todo!()
                }
            }
        },
        Body::Enum(body) => {
            quote!();
            todo!()
        },
    }
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