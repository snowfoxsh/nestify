mod nested;
mod fish;
mod attrs;

use proc_macro::{TokenStream as TokenStream1};
use proc_macro2::{TokenStream as TokenStream2};
use quote::quote;
use syn::parse_macro_input;
use crate::attrs::RecAttribute;
use crate::nested::{NestedStruct, NestedType};

/// use the nest macro to define structs with nested items as types.
/// its best to learn by example!
///
///
/// # Examples
///
/// ### Standard Syntax
/// you can define structures just like you can with standard rust syntax
/// ```
/// nestify::nest! {
///     struct MyStruct {
///         a: i32,
///         b: i32,
///     }
/// }
/// ```
/// like you would expect this compiles to
/// ```rust
/// struct MyStruct {
///     a: i32,
///     b: i32,
/// }
/// ```
///
/// ### Basic Nesting
/// here is where the actually functionality comes in.
/// you can define structures where you can define types
/// ```rust
/// nestify::nest! {
///    struct Outer {
///         one: struct Inner {
///             sub: i32,
///         },
///         two: f64,
///     }
/// }
/// ```
/// this code will compile to the following
/// ```rust
/// struct Outer {
///     one: Inner,
///     two: f64,
/// }
/// struct Inner {
///     sub: i32,
/// }
/// ```
///
/// ### Nested Generics
/// when defining nested generics you need to add generics to types. enter "fish" syntax.
/// to define generics on the field write ::<...>.
/// this will let you specify the nested generic types.
/// use fish syntax when the compiler gives an error such as
/// "missing generics for struct"
/// ```rust
/// nestify::nest! {
///     struct Outer {
///         f::<bool> : struct Inner<T> {
///             f: T
///         }
///     }
/// }
/// ```
/// this will compile to the following
/// ```rust
/// struct Outer {
///     f: Inner<bool>,
/// }
/// struct Inner<T> {
///     f: T,
/// }
/// ```
///
/// ### Attributes
/// adding attributes to your items is made easy with recursive attributes.
/// attributes can be defined as normal
/// ```rust
/// nestify::nest! {
///     #[derive(Clone)]
///     struct CloneMe {}
/// }
/// let x = CloneMe {};
/// let cl = x.clone();
/// ```
/// you can use star syntax to specify if an attribute is applied to all nested attributes.
/// for example
/// ```rust
/// nestify::nest! {
///     #[apply_all]*
///     #[apply_this]
///     struct GrandParent {
///         parent: struct Parent {
///             child: struct Child {
///                 payload: ()
///             }
///         }
///     }
/// }
/// ```
/// this compiles to
/// ```rust
/// #[apply_all]
/// #[apply_this]
/// struct GrandParent {
///     parent: Parent,
/// }
///
/// #[apply_all]
/// struct Parent {
///     child: Child,
/// }
///
/// #[apply_all]
/// struct Child {
///     payload: (),
/// }
/// ```

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
