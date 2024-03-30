use crate::special_data::{Body, FieldsNamed, FieldsUnnamed, Special, SpecialFields};
use crate::ty::SpecialType;
use crate::unpack_context::UnpackContext;
use proc_macro2::TokenStream;
use quote::quote;
use crate::attributes::CompositeAttribute;

/// A trait for types that can be unpacked within the context of custom attribute processing.
///
/// Implementors of this trait can be "unpacked" to generate Rust code (as `TokenStream`)
/// based on their structure and annotations, potentially including modifications
/// influenced by a provided `UnpackContext`.
pub(crate) trait Unpack {
    type Output;

    /// Unpacks the current structure into a Rust `TokenStream`, taking into account
    /// modifications from the given `UnpackContext` and any additional attributes.
    ///
    /// # Parameters
    /// - `self`: The instance of the implementor to unpack.
    /// - `context`: The unpacking context carrying information about inherited attributes
    ///   and possibly influencing how unpacking is performed.
    /// - `next`: A collection of `CompositeAttribute` that may modify the behavior of
    ///   unpacking or influence the generated output.
    ///
    /// # Returns
    /// `Self::Output`: The generated Rust code as a `TokenStream`.
    fn unpack(self, context: UnpackContext, next: Vec<CompositeAttribute>) -> Self::Output;
}

impl Unpack for Special {
    type Output = TokenStream;

    /// Performs unpacking for `Special` structures, handling struct and enum definitions
    /// uniquely based on their form and attributes.
    ///
    /// This function combines current and inherited attributes, applies any context-specific
    /// modifications, and generates a `TokenStream` representing the Rust code structure of
    /// the unpacked `Special` instance.
    ///
    /// # Parameters
    /// - `self`: The `Special` instance to be unpacked.
    /// - `unpack_context`: The context that may influence how unpacking is performed, including
    ///   attribute modifications.
    /// - `Next`: Additional attributes that may come from higher-level structures or previous
    ///   unpacking stages, to be considered in the current unpacking process.
    ///
    /// # Returns
    /// A `TokenStream` representing the generated Rust code after unpacking.
    fn unpack(
        self,
        mut unpack_context: UnpackContext,
        next: Vec<CompositeAttribute>,
    ) -> Self::Output {
        // combine the attributes from the current and previous
        let attrs = [self.attrs, next].concat();
        let attrs = unpack_context.modify_composite(attrs);

        let visibility = self.vis;
        let ident = self.ident; // the definition name/type
        let generics = self.generics;
        let where_clause = &generics.where_clause;
        // based on the type of the Special type [struct | enum | union?]
        // then determine the expansion
        match self.body {
            Body::Struct(body_struct) => match body_struct.fields {
                SpecialFields::Named(named) => {
                    let (body, definitions) = named.unpack(unpack_context, Vec::default());

                    // define our current ctx struct
                    // - define attributes
                    // - define ident and specify generics
                    // - insert our previous definitions behind the struct
                    quote!(
                        #(#attrs)*
                        #visibility struct #ident #generics #where_clause #body

                        #(#definitions)*
                    )
                }
                SpecialFields::Unnamed(unnamed) => {
                    // unpack our unnamed structure body, also collecting the recursive definitions
                    let (body, definitions) = unnamed.unpack(unpack_context, Vec::default());

                    quote!(
                        #(#attrs)*
                        #visibility struct #ident #generics #body #where_clause;

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
            },
            Body::Enum(body_enum) => {
                let mut accumulated_definitions = vec![];
                let mut variants = vec![];

                for variant in body_enum.variants {
                    let (attrs, next) = UnpackContext::filter_field_nested(variant.attrs); // todo: handle this
                    let ident = variant.ident;
                    let (field_body, mut definitions) =
                        variant.fields.unpack(unpack_context.clone(), next);
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
                    #visibility enum #ident #generics #where_clause {
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
    fn unpack(self, unpack_context: UnpackContext, next: Vec<CompositeAttribute>) -> Self::Output {
        match self {
            // Delegates to the `unpack` implementation of `FieldsNamed`, which handles the
            // unpacking of named fields,
            // including generating the necessary code and collecting
            // any additional definitions.
            SpecialFields::Named(named) => named.unpack(unpack_context, next),

            // Similarly, for unnamed fields (tuples), it delegates to `FieldsUnnamed`'s
            // `unpack` method, which is specialized in handling tuple-like structures.
            SpecialFields::Unnamed(unnamed) => unnamed.unpack(unpack_context, next),

            // For unit types, which have no fields, the function returns a default (empty)
            // `TokenStream` along with an empty vector for definitions,
            // as there's no additional
            // code needed to represent a unit type in Rust.
            SpecialFields::Unit => (TokenStream::default(), Vec::<TokenStream>::default()),
        }
    }
}

impl Unpack for FieldsNamed {
    type Output = (TokenStream, Vec<TokenStream>);
    //             ^body        ^definitions
    fn unpack(
        self,
        unpack_context: UnpackContext,
        from_variant: Vec<CompositeAttribute>,
    ) -> Self::Output {
        // fields buffer load each
        let mut fields = vec![];
        let mut definitions = vec![];

        // iterate through the fields
        for field in self.named {
            // filter the attributes, passing the #> to the next iteration,
            // we need to filter the attributes so that we can determine which are normal
            // or which should be passed on
            let (attrs, next) = UnpackContext::filter_field_nested(field.attrs);
            let vis = field.vis;
            // unused field mutability see syn doc for FieldMutability
            let _mutability = field.mutability;
            // this is a named type, so there should always be an ident
            // if there is no ident then there should be a parsing bug
            let ident = field.ident.unwrap_or_else(|| {
                panic!(
                    "Internal Macro Error. This is a bug. \
                Please Consider opening an issue with steps to reproduce the bug \
                Provide this information: Error from line {}",
                    { line!() }
                )
            });

            let fish = field.fish;

            // branch off the type depending on if leaf is reached
            match field.ty {
                // leaf node aka a non-special type => don't recurse
                // `SpecialType::Type`
                // doesn't need fish because it will always be None
                SpecialType::Type(ty) => {
                    let field = quote!(
                        #(#attrs)*
                        #vis #ident : #ty
                    );
                    fields.push(field);
                }
                SpecialType::Augmented(augmented) => {
                    // combine attributes possibly inherited from an enum variant with field attrs
                    let next = [next, from_variant.clone()].concat();

                    let (ty, mut aug_definitions) = augmented.unpack(unpack_context.clone(), next);
                    definitions.append(&mut aug_definitions);

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
                        #vis #ident : #ty #fish
                    );
                    fields.push(field);

                    // combine attributes possibly inherited from an enum variant with field attrs
                    let next = [next, from_variant.clone()].concat();

                    // unpack the definition of the type
                    // then add it to the definition buffer
                    // this could be one or more definition
                    // we don't care
                    let definition = special.unpack(unpack_context.clone(), next);
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
    fn unpack(
        self,
        unpack_context: UnpackContext,
        from_variant: Vec<CompositeAttribute>,
    ) -> Self::Output {
        let mut fields = vec![];
        let mut definitions = vec![];

        // iterate through types
        for field in self.unnamed {
            // filter the attributes, passing the #> to the next iteration
            let (attrs, next) = UnpackContext::filter_field_nested(field.attrs);
            let vis = field.vis;

            // unused field mutability see syn doc for FieldMutability
            let _mutability = field.mutability;

            // this is an unnamed variant so there should never Some(T)
            let _ident = field.ident; // todo: warn if this is not none

            let fish = field.fish;

            // branch off based on if a type is defined or should be defined
            match field.ty {
                SpecialType::Type(ty) => {
                    let field = quote!(
                        #(#attrs)*
                        #vis #ty
                    );
                    fields.push(field);
                }
                SpecialType::Augmented(augmented) => {
                    // combine attributes possibly inherited from an enum variant with field attrs
                    let next = [next, from_variant.clone()].concat();

                    let (ty, mut aug_definitions) = augmented.unpack(unpack_context.clone(), next);
                    definitions.append(&mut aug_definitions);

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
                        #vis #ty #fish
                    );
                    fields.push(field);

                    // combine attributes possibly inherited from an enum variant with field attrs
                    let next = [next, from_variant.clone()].concat();

                    let definition = special.unpack(unpack_context.clone(), next);
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
