use proc_macro2::TokenStream;
use quote::quote;
use crate::special_data::{Body, FieldsNamed, FieldsUnnamed, Special, SpecialFields, SpecialVariant};
use crate::ty::SpecialType;

// this is all one nasty function but i think its the best way
// todo: add attribute support
pub fn unpack(def: Special) -> proc_macro2::TokenStream {
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
                                fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                ));
                            }
                            // recuse down the parse stack
                            SpecialType::Def(special) => {
                                // trust that ty will be a defintion step
                                let ty = &special.ident; // dont move so no clone!
                                fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                ));

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
                        #visablity struct #ident #generics {
                            #(#fields),*
                        }

                        #(#definitions)*
                    )
                }


                // unpack a tuple struct or tuple variant
                // todo [change] remove the semicolon
                SpecialFields::Unnamed(unnamed) => {
                    let mut fields = vec![];
                    let mut definitions = vec![];

                    // iterate through types
                    for field in unnamed.unnamed {
                        let attrs = field.attrs;
                        let vis = field.vis;

                        // unused field mutability see syn doc for FieldMutability
                        let _mutability = field.mutability;

                        // this is an unnamed variant so there should never Some(T)
                        let _ident = field.ident; // todo: warn if this is not none

                        // branch off based on if type is defined or should be defined
                        match field.ty {
                            SpecialType::Type(ty) => {
                                fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ty
                                ));
                            }
                            SpecialType::Def(special) => {
                                let ty = &special.ident;

                                fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ty
                                ));

                                let definition = unpack(special);
                                definitions.push(definition);
                            }
                        }
                    }

                    quote!(
                        // todo attrs
                        #visablity struct #ident #generics (
                            #(#fields),*
                        );

                        #(#definitions)*
                    )
                }
                SpecialFields::Unit => {
                    quote!(
                        #visablity struct #ident #generics;
                    )
                }
            }
        },
        Body::Enum(body) => {
            quote!();
            todo!()
        },
    }
}

pub(crate) trait Unpack {
    type Output;
    fn unpack(self) -> Self::Output;
}

impl Unpack for Special {

    type Output = TokenStream;
    fn unpack(self) -> Self::Output {
        let attrs = self.attrs;
        let visablity = self.vis;
        let ident = self.ident; // the definition name/type
        let generics = self.generics;

        // based on the type of the Special type [struct | enum | union?]
        // then determine the expansion
        match self.body {
            Body::Struct(body) => match body.fields {
                SpecialFields::Named(named) => {
                    let (fields, definitions) = named.unpack();

                    // define our current ctx struct
                    // - define attributes
                    // - define ident and specify generics
                    // - insert our previous definitions behind the struct
                    quote!(
                        // todo attrs
                        #visablity struct #ident #generics {
                            #(#fields),*
                        }

                        #(#definitions)*
                    )
                }
                SpecialFields::Unnamed(unnamed) => {
                    let (fields, definitions) = unnamed.unpack();

                    quote!(
                        // todo attrs
                        #visablity struct #ident #generics (
                            #(#fields),*
                        );

                        #(#definitions)*
                    )
                }
                SpecialFields::Unit => {
                    quote!(
                        #visablity struct #ident #generics;
                    )
                }
            }
            Body::Enum(body) => {
                for variant in body.variants {
                    match variant {
                        SpecialVariant { .. } => {}
                    }
                }
                todo!()
            }
        }
    }
}


impl Unpack for SpecialFields {
    type Output = TokenStream;
    fn unpack(self) -> Self::Output {
        match self {
            SpecialFields::Named(_) => {todo!()}
            SpecialFields::Unnamed(_) => {todo!()}
            SpecialFields::Unit => {todo!()}
        }
    }
}

impl Unpack for FieldsNamed {
    type Output = (Vec<TokenStream>, Vec<TokenStream>);
    fn unpack(self) -> Self::Output {
        // fields buffer load each
        let mut fields = vec![];
        let mut definitions = vec![];

        // iterate through the fields
        for field in self.named {
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
                    fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                ));
                }
                // recuse down the parse stack
                SpecialType::Def(special) => {
                    // trust that ty will be a defintion step
                    let ty = &special.ident; // dont move so no clone!
                    fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ident : #ty
                                    // todo: add fish syntax
                                ));

                    // unpack the definition of the type
                    // then add it to the definition buffer
                    // this could be one or more definition
                    // we don't care
                    let definition = special.unpack();
                    definitions.push(definition);
                }
            }
        }

        (fields, definitions)
    }
}

impl Unpack for FieldsUnnamed {
    type Output = (Vec<TokenStream>, Vec<TokenStream>);
    fn unpack(self) -> Self::Output {
        let mut fields = vec![];
        let mut definitions = vec![];

        // iterate through types
        for field in self.unnamed {
            let attrs = field.attrs;
            let vis = field.vis;

            // unused field mutability see syn doc for FieldMutability
            let _mutability = field.mutability;

            // this is an unnamed variant so there should never Some(T)
            let _ident = field.ident; // todo: warn if this is not none

            // branch off based on if type is defined or should be defined
            match field.ty {
                SpecialType::Type(ty) => {
                    fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ty
                                ));
                }
                SpecialType::Def(special) => {
                    let ty = &special.ident;

                    fields.push(quote!(
                                    // #(#attrs)* todo
                                    #vis #ty
                                ));

                    let definition = special.unpack();
                    definitions.push(definition);
                }
            }
        }

        (fields, definitions)
    }
}