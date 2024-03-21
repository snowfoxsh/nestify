use syn::{parse_str, Token};
use quote::quote;
use crate::attributes::AttributeModifier;

#[test]
fn parse_star_modifier() {
    let input = quote!{*};
    let parsed = parse_str::<>(&input.to_string());

    assert!(matches!(parsed, Ok(AttributeModifier::Star(_))));
}

#[test]
fn parse_slash_modifier() {
    let input = quote!{/};
    let parsed = parse_str::<AttributeModifier>(&input.to_string());

    assert!(matches!(parsed, Ok(AttributeModifier::Slash(_))));
}

#[test]
fn parse_minus_modifier() {
    let input = quote!{-};
    let parsed = parse_str::<AttributeModifier>(&input.to_string());

    assert!(matches!(parsed, Ok(AttributeModifier::Minus(_))));
}

#[test]
fn parse_plus_modifier_with_depth() {
    let input = quote!{+ 5};
    let parsed = parse_str::<AttributeModifier>(&input.to_string()).expect("Should parse correctly");

    match parsed {
        AttributeModifier::Plus { depth, .. } => assert_eq!(depth, 5),
        _ => panic!("Expected Plus variant"),
    }
}

#[test]
fn parse_plus_modifier_with_invalid_depth() {
    let input = quote!{+not_a_number};
    let parsed = parse_str::<AttributeModifier>(&input.to_string());

    assert!(parsed.is_err()); // Expecting an error due to invalid depth
}

