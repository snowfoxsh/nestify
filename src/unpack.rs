use proc_macro2::TokenStream;
use quote::quote;
use crate::special_data::{Body, FieldsNamed, FieldsUnnamed, Special, SpecialFields, SpecialVariant};
use crate::ty::SpecialType;

pub(crate) trait Unpack {
    type Output;
    fn unpack(self) -> Self::Output;
}

impl Unpack for Special {

    type Output = TokenStream;
    fn unpack(self) -> Self::Output {
        let attrs = self.attrs;
        let visibility = self.vis;
        let ident = self.ident; // the definition name/type
        let generics = self.generics;

        // based on the type of the Special type [struct | enum | union?]
        // then determine the expansion
        match self.body {
            Body::Struct(body_struct) => match body_struct.fields {
                SpecialFields::Named(named) => {
                    let (body, definitions) = named.unpack();

                    // define our current ctx struct
                    // - define attributes
                    // - define ident and specify generics
                    // - insert our previous definitions behind the struct
                    quote!(
                        #(#attrs)*
                        #visibility struct #ident #generics #body

                        #(#definitions)*
                    )
                }
                SpecialFields::Unnamed(unnamed) => {
                    // unpack our unnamed structure body, also collecting the recursive definitions
                    let (body, definitions) = unnamed.unpack();

                    quote!(
                        #(#attrs)*
                        #visibility struct #ident #generics #body;

                        #(#definitions)*
                    )
                }
                SpecialFields::Unit => {
                    // no unpacking required here, since there are no types
                    // in other words, this branch is always a leaf
                    quote!(
                        #(#attrs)*
                        #visibility struct #ident #generics;
                    )
                }
            }
            Body::Enum(body_enum) => {
                let mut accumulated_definitions = vec![];
                let mut variants = vec![];
                
                for variant in body_enum.variants {
                    let attrs = variant.attrs;
                    let ident = variant.ident;
                    let (field_body, mut definitions) = variant.fields.unpack();
                    accumulated_definitions.append(&mut definitions);
                    // todo: get variant working
                    let discriminant = variant.discriminant;
                    
                    let variant = quote!(
                        #(#attrs)*
                        #ident #field_body
                        #discriminant
                    );
                    variants.push(variant);
                }
                
                quote!(
                    #(#attrs)*
                    #visibility enum #ident #generics {
                        #( #variants ),*
                    }
                    
                    #(#accumulated_definitions)*
                )
            }
        }
    }
}


impl Unpack for SpecialFields {
    type Output = (TokenStream, Vec<TokenStream>);
    //             ^body        ^definitions
    fn unpack(self) -> Self::Output {
        match self {
            SpecialFields::Named(named) => named.unpack(),
            SpecialFields::Unnamed(unnamed) => unnamed.unpack(),
            SpecialFields::Unit => (TokenStream::default(), Vec::<TokenStream>::default())
        }
    }
}

impl Unpack for FieldsNamed {
    type Output = (TokenStream, Vec<TokenStream>);
    //             ^body        ^definitions
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
                    // todo: add fish syntax
                    let field = quote!(
                        #(#attrs)* 
                        #vis #ident : #ty
                    );
                    fields.push(field);
                }
                // recuse down the parse stack
                SpecialType::Def(special) => {
                    // trust that ty will be a definition step
                    let ty = &special.ident; // don't move so no clone!

                    // todo: add fish syntax
                    let field = quote!(
                        #(#attrs)* 
                        #vis #ident : #ty
                    );
                    fields.push(field);

                    // unpack the definition of the type
                    // then add it to the definition buffer
                    // this could be one or more definition
                    // we don't care
                    let definition = special.unpack();
                    definitions.push(definition);
                }
            }
        }
        
        let body = quote!(
            { #(#fields),* }
        );

        (body, definitions)
    }
}

impl Unpack for FieldsUnnamed {
    type Output = (TokenStream, Vec<TokenStream>);
    //             ^body        ^definitions
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

            // branch off based on if a type is defined or should be defined
            match field.ty {
                SpecialType::Type(ty) => {
                    let field = quote!(
                        #(#attrs)* 
                        #vis #ty
                    );
                    fields.push(field);
                }
                SpecialType::Def(special) => {
                    let ty = &special.ident;

                    let field = quote!(
                        #(#attrs)*
                        #vis #ty
                    );
                    fields.push(field);
                    
                    let definition = special.unpack();
                    definitions.push(definition);
                }
            }
        }
        
        let body = quote!(
            ( #(#fields),* )
        );
        
        (body, definitions)
    }
}