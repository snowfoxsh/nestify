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
