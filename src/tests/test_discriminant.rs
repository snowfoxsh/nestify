use quote::quote;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Expr, parse_quote};
use crate::discriminant::Discriminant;

#[test]
fn discriminant_with_simple_expression() {
    let expr: Expr = parse_quote! { 42 };
    let discriminant = Discriminant {
        eq_token: parse_quote! { = },
        expr,
    };

    let mut tokens = TokenStream::new();
    discriminant.to_tokens(&mut tokens);

    let expected_output = quote! { = 42 };
    assert_eq!(tokens.to_string(), expected_output.to_string(), "Simple expression not correctly tokenized");
}

#[test]
fn discriminant_with_complex_expression() {
    let expr: Expr = parse_quote! { 4 + 20 };
    let discriminant = Discriminant {
        eq_token: parse_quote! { = },
        expr,
    };

    let mut tokens = TokenStream::new();
    discriminant.to_tokens(&mut tokens);

    let expected_output = quote! { = 4 + 20 };
    assert_eq!(tokens.to_string(), expected_output.to_string(), "Complex expression not correctly tokenized");
}

#[test]
fn discriminant_with_function_call() {
    let expr: Expr = parse_quote! { my_function(42, "hello".to_string()) };
    let discriminant = Discriminant {
        eq_token: parse_quote! { = },
        expr,
    };

    let mut tokens = TokenStream::new();
    discriminant.to_tokens(&mut tokens);

    let expected_output = quote! { = my_function(42, "hello".to_string()) };
    assert_eq!(tokens.to_string(), expected_output.to_string(), "Function call expression not correctly tokenized");
}

#[test]
fn discriminant_with_nested_expression() {
    let expr: Expr = parse_quote! { (3 + 7) * 2 };
    let discriminant = Discriminant {
        eq_token: parse_quote! { = },
        expr,
    };

    let mut tokens = TokenStream::new();
    discriminant.to_tokens(&mut tokens);

    let expected_output = quote! { = (3 + 7) * 2 };
    assert_eq!(tokens.to_string(), expected_output.to_string(), "Nested expression not correctly tokenized");
}