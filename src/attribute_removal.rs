use crate::attributes::Attribute;
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{Meta, MetaList};

enum AttributeRemoval {
    NoMatch,
    RemoveWhole,
    ReplaceWith(Meta),
}

enum ExistingAfter {
    RemoveWhole,
    ReplaceWith(Meta),
}

struct Consumption {
    existing_after: ExistingAfter,
    remaining_removal: Option<Meta>,
}

fn token_key(tokens: impl ToTokens) -> String {
    tokens.to_token_stream().to_string()
}

fn same_path(left: &syn::Path, right: &syn::Path) -> bool {
    token_key(left) == token_key(right)
}

fn same_delimiter(left: &syn::MacroDelimiter, right: &syn::MacroDelimiter) -> bool {
    matches!(
        (left, right),
        (syn::MacroDelimiter::Paren(_), syn::MacroDelimiter::Paren(_))
            | (syn::MacroDelimiter::Bracket(_), syn::MacroDelimiter::Bracket(_))
            | (syn::MacroDelimiter::Brace(_), syn::MacroDelimiter::Brace(_))
    )
}

fn is_cfgattr(path: &syn::Path) -> bool {
    path.is_ident("cfg_attr")
}

fn split_commas(tokens: TokenStream) -> Vec<TokenStream> {
    let mut items = Vec::new();
    let mut current = TokenStream::new();

    for token in tokens {
        match &token {
            TokenTree::Punct(punct) if punct.as_char() == ',' => {
                if !current.is_empty() {
                    items.push(current);
                }

                current = TokenStream::new();
            }
            _ => {
                current.extend(std::iter::once(token));
            }
        }
    }

    if !current.is_empty() {
        items.push(current);
    }

    items
}

fn join_comma_sep(items: Vec<TokenStream>) -> TokenStream {
    let mut tokens = TokenStream::new();

    for (idx, item) in items.into_iter().enumerate() {
        if idx > 0 {
            tokens.extend(quote!(,));
        }

        tokens.extend(item);
    }

    tokens
}

fn parse_meta(tokens: &TokenStream) -> Option<Meta> {
    syn::parse2::<Meta>(tokens.clone()).ok()
}

fn rebuild_existing_meta_list(original: &MetaList, items: Vec<TokenStream>) -> Option<Meta> {
    if items.is_empty() {
        return None;
    }

    // cfg_attr(condition) alone is invalid/useless after payload removal
    if is_cfgattr(&original.path) && items.len() <= 1 {
        return None;
    }

    Some(Meta::List(MetaList {
        path: original.path.clone(),
        delimiter: original.delimiter.clone(),
        tokens: join_comma_sep(items),
    }))
}

fn rebuild_remaining_meta_list(original: &MetaList, items: Vec<TokenStream>) -> Option<Meta> {
    if items.is_empty() {
        return None;
    }

    Some(Meta::List(MetaList {
        path: original.path.clone(),
        delimiter: original.delimiter.clone(),
        tokens: join_comma_sep(items),
    }))
}

fn consume_item_from_items(
    items: &mut Vec<TokenStream>,
    removal_item: TokenStream,
    min_existing_index: usize,
) -> Option<Option<TokenStream>> {
    // None => this item could not be consumed at all
    // Some(None) => this item was fully consumed
    // Some(Some(tokens)) => this item was partially consumed. tokens remain

    let mut remaining = removal_item;
    let mut changed = false;

    loop {
        let remaining_key = token_key(&remaining);

        if let Some(idx) = (min_existing_index..items.len())
            .find(|&idx| token_key(&items[idx]) == remaining_key)
        {
            items.remove(idx);
            return Some(None);
        }

        let Some(removal_meta) = parse_meta(&remaining) else {
            return if changed { Some(Some(remaining)) } else { None };
        };

        let mut found = false;

        for idx in min_existing_index..items.len() {
            let Some(existing_meta) = parse_meta(&items[idx]) else {
                continue;
            };

            let Some(consumption) = consume_meta(&existing_meta, &removal_meta) else {
                continue;
            };

            changed = true;
            found = true;

            match consumption.existing_after {
                ExistingAfter::RemoveWhole => {
                    items.remove(idx);
                }
                ExistingAfter::ReplaceWith(meta) => {
                    items[idx] = meta.to_token_stream();
                }
            }

            match consumption.remaining_removal {
                None => return Some(None),
                Some(meta) => {
                    remaining = meta.to_token_stream();
                    break;
                }
            }
        }

        if !found {
            return if changed { Some(Some(remaining)) } else { None };
        }
    }
}

fn consume_meta(existing: &Meta, removal: &Meta) -> Option<Consumption> {
    if token_key(existing) == token_key(removal) {
        return Some(Consumption {
            existing_after: ExistingAfter::RemoveWhole,
            remaining_removal: None,
        });
    }

    let (Meta::List(existing_list), Meta::List(removal_list)) = (existing, removal) else {
        return None;
    };

    if !same_path(&existing_list.path, &removal_list.path) {
        return None;
    }

    if !same_delimiter(&existing_list.delimiter, &removal_list.delimiter) {
        return None;
    }

    let mut existing_items = split_commas(existing_list.tokens.clone());
    let removal_items = split_commas(removal_list.tokens.clone());

    if existing_items.is_empty() || removal_items.is_empty() {
        return None;
    }

    // for cfg_attr, the first item is the condition
    let existing_payload_start = if is_cfgattr(&existing_list.path) {
        1
    } else {
        0
    };

    // cfg_attr(derive(Debug)) is shorthand
    // cfg_attr(all(), derive(Debug)) targets a specific condition
    let removal_has_explicit_cfg_condition =
        is_cfgattr(&removal_list.path) && removal_items.len() >= 2;

    if removal_has_explicit_cfg_condition {
        let existing_condition = existing_items.first()?;
        let removal_condition = &removal_items[0];

        if token_key(existing_condition) != token_key(removal_condition) {
            return None;
        }
    }

    let removal_payload_start = if removal_has_explicit_cfg_condition {
        1
    } else {
        0
    };

    let removal_payload_items = removal_items[removal_payload_start..].to_vec();

    if removal_payload_items.is_empty() {
        return None;
    }

    let mut changed = false;
    let mut remaining_payload_items = Vec::new();

    for removal_item in removal_payload_items {
        match consume_item_from_items(
            &mut existing_items,
            removal_item.clone(),
            existing_payload_start,
        ) {
            None => {
                remaining_payload_items.push(removal_item);
            }
            Some(None) => {
                changed = true;
            }
            Some(Some(remaining_item)) => {
                changed = true;
                remaining_payload_items.push(remaining_item);
            }
        }
    }

    if !changed {
        return None;
    }

    let existing_after = match rebuild_existing_meta_list(existing_list, existing_items) {
        Some(meta) => ExistingAfter::ReplaceWith(meta),
        None => ExistingAfter::RemoveWhole,
    };

    let remaining_removal = if remaining_payload_items.is_empty() {
        None
    } else {
        let mut remaining_items = Vec::new();

        if removal_has_explicit_cfg_condition {
            remaining_items.push(removal_items[0].clone());
        }

        remaining_items.extend(remaining_payload_items);

        rebuild_remaining_meta_list(removal_list, remaining_items)
    };

    Some(Consumption {
        existing_after,
        remaining_removal,
    })
}

pub fn remove_or_subtract_attr(stack: &mut Vec<Attribute>, removal: &Attribute) -> bool {
    let original_stack = stack.clone();
    let mut remaining = removal.meta.clone();

    let mut idx = stack.len();

    while idx > 0 {
        idx -= 1;

        let Some(consumption) = consume_meta(&stack[idx].meta, &remaining) else {
            continue;
        };

        match consumption.existing_after {
            ExistingAfter::RemoveWhole => {
                stack.remove(idx);
            }
            ExistingAfter::ReplaceWith(meta) => {
                stack[idx].meta = meta;
            }
        }

        match consumption.remaining_removal {
            None => return true,
            Some(meta) => {
                remaining = meta;
            }
        }
    }

    // partial failure must not mutate the stack.
    *stack = original_stack;
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Meta;

    fn meta(input: &str) -> Meta {
        syn::parse_str::<Meta>(input).unwrap()
    }

    fn subtract_meta(existing: &Meta, removal: &Meta) -> AttributeRemoval {
        let Some(consumption) = consume_meta(existing, removal) else {
            return AttributeRemoval::NoMatch;
        };

        if consumption.remaining_removal.is_some() {
            return AttributeRemoval::NoMatch;
        }

        match consumption.existing_after {
            ExistingAfter::RemoveWhole => AttributeRemoval::RemoveWhole,
            ExistingAfter::ReplaceWith(meta) => AttributeRemoval::ReplaceWith(meta),
        }
    }

    // we use this because of weird TokenStream::to_string()s
    fn assert_replace_eq(result: AttributeRemoval, expected: &str) {
        let expected = syn::parse_str::<Meta>(expected).unwrap();

        match result {
            AttributeRemoval::ReplaceWith(actual) => {
                assert_eq!(token_key(actual), token_key(expected));
            }
            AttributeRemoval::RemoveWhole => {
                panic!("expected replacement, got whole-attr removal");
            }
            AttributeRemoval::NoMatch => {
                panic!("expected replacement, got no match");
            }
        }
    }

    #[test]
    fn assert_basic_removal() {
        let existing = meta("derive(Debug, Clone)");
        let removal = meta("derive(Debug)");

        assert_replace_eq(
            subtract_meta(&existing, &removal),
            "derive(Clone)",
        );
    }

    #[test]
    fn assert_cfg_attr_removal() {
        let existing = meta("cfg_attr(all(), derive(Debug, Eq, PartialEq), derive(Clone))");
        let removal = meta("cfg_attr(derive(Debug, Eq, Clone))");

        assert_replace_eq(
            subtract_meta(&existing, &removal),
            "cfg_attr(all(), derive(PartialEq))",
        );
    }

    #[test]
    fn assert_rdoc_removal() {
        let existing = meta(r#"doc = "hello""#);
        let removal = meta(r#"doc = "hello""#);

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::RemoveWhole
        ));
    }

    #[test]
    fn assert_no_partial_rdoc_removal() {
        let existing = meta(r#"doc = "hello""#);
        let removal = meta(r#"doc = "hell""#);

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::NoMatch
        ));
    }

    #[test]
    fn assert_no_cross_match_removal() {
        let existing = meta("foo(a, b)");
        let removal = syn::parse_str::<Meta>("foo[a]").unwrap();

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::NoMatch
        ));
    }

    #[test]
    fn assert_cfg_attr_condition_exact_match() {
        let existing = meta("cfg_attr(all(), derive(Debug))");
        let removal = meta("cfg_attr(any(), derive(Debug))");

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::NoMatch
        ));
    }

    #[test]
    fn assert_no_cfg_attr_condition_removal() {
        let existing = meta("cfg_attr(all(unix, windows), derive(Debug))");
        let removal = meta("cfg_attr(all(unix))");

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::NoMatch
        ));
    }

    #[test]
    fn assert_whole_bracket_attr_removal() {
        let existing = meta("foo[bar, baz]");
        let removal = meta("foo[bar, baz]");

        assert!(matches!(
            subtract_meta(&existing, &removal),
            AttributeRemoval::RemoveWhole
        ));
    }

    #[test]
    fn assert_attr_with_path_removal() {
        let existing = meta("clippy::allow(dead_code, unused_variables)");
        let removal = meta("clippy::allow(dead_code)");

        assert_replace_eq(
            subtract_meta(&existing, &removal),
            "clippy::allow(unused_variables)",
        );
    }

    #[test]
    fn assert_attr_with_path_and_args_removal() {
        let existing = meta(r#"custom("literal", flag)"#);
        let removal = meta(r#"custom("literal")"#);

        assert_replace_eq(
            subtract_meta(&existing, &removal),
            "custom(flag)",
        );
    }

    #[test]
    fn assert_nested_custom_attr_removal() {
        let existing = meta("custom(foo(1, 10), flag)");
        let removal = meta("custom(foo(1))");

        assert_replace_eq(
            subtract_meta(&existing, &removal),
            "custom(foo(10), flag)",
        );
    }

    #[test]
    fn assert_failed_removal_rolls_back() {
        let mut stack = vec![
            Attribute { pound_token: Default::default(), bracket_token: Default::default(), meta: meta("cfg_attr(all(), derive(Debug, PartialEq))") },
            Attribute { pound_token: Default::default(), bracket_token: Default::default(), meta: meta("cfg_attr(all(), derive(Clone))") },
        ];

        let original = stack.clone();

        let removal = Attribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            meta: meta("cfg_attr(derive(Debug, Clone, Eq))"),
        };

        assert!(!remove_or_subtract_attr(&mut stack, &removal));

        assert_eq!(
        stack.iter().map(|attr| token_key(&attr.meta)).collect::<Vec<_>>(),
        original.iter().map(|attr| token_key(&attr.meta)).collect::<Vec<_>>(),
    );
    }
}