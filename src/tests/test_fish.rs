//! Tests for [FishHook](crate::fish::FishHook)

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse2, parse_quote};
use crate::fish::FishHook;

#[test]
fn parse_valid_fishhook() {
    let input = quote! { ||<T> };
    let fish_hook: Result<FishHook, syn::Error> = syn::parse2(input);

    assert!(fish_hook.is_ok(), "Failed to parse a valid FishHook");

    if let Ok(fish) = fish_hook {
        assert!(fish.generics.params.iter().count() > 0, "FishHook should have generics");
    }
}

#[test]
fn parse_fishhook_without_generics() {
    let input = quote! { || };
    let fish_hook: Result<FishHook, syn::Error> = parse2(input);

    assert!(fish_hook.is_err(), "Parsed a FishHook without generics");
}

#[test]
fn parse_fishhook_with_complex_generics() {
    let input = quote! { ||<T: Clone + Default, U: 'static + Sync> };
    let fish_hook: Result<FishHook, syn::Error> = parse2(input);

    assert!(fish_hook.is_ok(), "Failed to parse FishHook with complex generics");

    if let Ok(fish) = fish_hook {
        let generics_count = fish.generics.params.iter().count();
        assert_eq!(generics_count, 2, "FishHook should have two generic parameters");
    }
}

#[test]
fn parse_fishhook_missing_prefix() {
    let input = quote! { <T> };
    let fish_hook: Result<FishHook, syn::Error> = parse2(input);

    assert!(fish_hook.is_err(), "Parsed FishHook missing the || prefix");
}

#[test]
fn parse_fishhook_with_incorrect_syntax() {
    let inputs = vec![
        quote! { ||<T: Clone + Default U: 'static + Sync> }, // Missing comma
        quote! { ||T> }, // Missing angle brackets
        quote! { ||<> }, // Empty generics
    ];

    for input in inputs {
        let result: Result<FishHook, syn::Error> = parse2(input);
        assert!(result.is_err(), "Parsed FishHook with incorrect syntax");
    }
}

#[test]
fn parse_fishhook_with_lifetime_and_constraints() {
    let input = quote! { ||<'a, T: 'a + Sync> };
    let fish_hook: Result<FishHook, syn::Error> = parse2(input);

    assert!(fish_hook.is_ok(), "Failed to parse FishHook with lifetime and constraints");

    if let Ok(fish) = fish_hook {
        // Ensure the generics include a lifetime and a type constraint
        let has_lifetime = fish.generics.params.iter().any(|p| matches!(p, syn::GenericParam::Lifetime(_)));
        let has_type_constraints = fish.generics.params.iter().any(|p| matches!(p, syn::GenericParam::Type(_)));

        assert!(has_lifetime, "Expected lifetime parameter in generics");
        assert!(has_type_constraints, "Expected type parameter with constraints in generics");
    }
}

#[test]
fn to_tokens_generates_correct_token_stream() {
    let fish_hook = FishHook {
        prefix: parse_quote! { || },
        generics: parse_quote! { <T> },
    };

    let mut token_stream = TokenStream::new();
    fish_hook.to_tokens(&mut token_stream);

    let expected_output = quote! { <T> }.to_string();
    let generated_output = token_stream.to_string();

    assert_eq!(generated_output, expected_output, "Generated TokenStream does not match expected output");
}
