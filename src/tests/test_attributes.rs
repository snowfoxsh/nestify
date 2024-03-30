use syn::{Error, parse_str};
use syn::parse::{Parse, ParseStream};
use crate::attributes::*;


/// Tests for [Attribute](crate::attributes::Attribute)
mod attribute {
    use super::*;

    #[test]
    fn parse_single_valid_attribute() {
        let input = "#[derive(Debug, Clone)]";
        let result: Result<Attribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_ok(), "Failed to parse a valid single attribute");

        // Further assertions can be done here based on the expected structure of `Attribute`
        if let Ok(attr) = result {
            assert_eq!(attr.meta.path().get_ident().unwrap().to_string(), "derive");
            // Additional checks based on `meta` content can be added here
        }
    }

    #[test]
    fn parse_multiple_valid_attributes() {
        let input = "#[derive(Debug)] #[test]"; // Example with multiple attributes
        let result: Result<Vec<Attribute>, Error> = parse_attribute_from_str_many(input);
        assert!(result.is_ok(), "Failed to parse multiple valid attributes");

        if let Ok(attrs) = result {
            assert_eq!(attrs.len(), 2);
            // Additional checks for each attribute
        }
    }

    #[test]
    fn parse_invalid_attribute() {
        let input = "#[]"; // Intentionally missing arguments
        let result: Result<Attribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");
        
        let input = "#derive"; // Intentionally missing []
        let result: Result<Attribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");
        
        let input = "#![meta]"; // ! not allowed
        let result: Result<Attribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");
    }
}

/// Tests for [NestedAttribute](crate::attributes::NestedAttribute)
mod nested_attribute {
    use super::*;

    #[test]
    fn parse_single_valid_nested_attribute() {
        // Assuming NestedAttribute expects a specific syntax, adjust accordingly
        let input = "#>[nested(Debug, Clone)]";
        let result: Result<NestedAttribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_ok(), "Failed to parse a valid single nested attribute");

        if let Ok(nested_attr) = result {
            // Assuming `NestedAttribute` also has a `meta` field or similar for assertions
            assert_eq!(nested_attr.meta.path().get_ident().unwrap().to_string(), "nested");
            // Further checks based on the structure of `NestedAttribute` can be added here
        }
    }

    #[test]
    fn parse_invalid_nested_attribute() {
        // Examples of invalid nested attribute inputs
        let inputs = vec![
            "#>[]", // Missing inner content
            "#>meta", // No Brackets
            "#[nested(Debug)]", // Missing '>' indicator for nested attributes
        ];

        for input in inputs {
            let result: Result<NestedAttribute, Error> = parse_attribute_from_str(input);
            assert!(result.is_err(), "Parsed an invalid nested attribute successfully");
        }
    }

    #[test]
    fn parse_multiple_valid_nested_attributes() {
        // Assuming a syntax for nested attributes, adjust this string as necessary.
        // This example assumes nested attributes are delineated similarly to top-level attributes.
        let input = "#>[nested(Debug)] #>[nested(Clone)]";
        let result: Result<Vec<NestedAttribute>, Error> = parse_attribute_from_str_many(input);
        assert!(result.is_ok(), "Failed to parse multiple valid nested attributes");

        if let Ok(nested_attrs) = result {
            assert_eq!(nested_attrs.len(), 2, "Expected two nested attributes to be parsed");
            // Assuming we can check the 'meta' or similar field for each NestedAttribute
            assert_eq!(nested_attrs[0].meta.path().get_ident().unwrap().to_string(), "nested");
            assert_eq!(nested_attrs[1].meta.path().get_ident().unwrap().to_string(), "nested");
            // Further checks based on the structure of `NestedAttribute` can be added here
        }
    }
}

/// Tests for [CompositeAttribute](crate::attributes::CompositeAttribute)
mod composite_attribute {
    use super::*;

    #[test]
    fn parse_single_valid_composite_attribute() {
        let input = "#[derive(Debug, Clone)]*";
        let result: Result<CompositeAttribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_ok(), "Failed to parse a valid single attribute");

        // Further assertions can be done here based on the expected structure of `Attribute`
        if let Ok(attr) = result {
            assert_eq!(attr.meta.path().get_ident().unwrap().to_string(), "derive");
            // Additional checks based on `meta` content can be added here
        }
    }

    #[test]
    fn parse_multiple_valid_composite_attributes() {
        let input = "#[one] #[two]* #[three]/"; // Example with multiple attributes
        let result: Result<Vec<CompositeAttribute>, Error> = parse_attribute_from_str_many(input);
        assert!(result.is_ok(), "Failed to parse multiple valid attributes");

        if let Ok(attrs) = result {
            assert_eq!(attrs.len(), 3);
            // Additional checks for each attribute
        }
    }

    #[test]
    fn parse_invalid_composite_attribute() {
        let input = "#[]*"; // Intentionally missing arguments
        let result: Result<CompositeAttribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");

        let input = "#derive"; // Intentionally missing []
        let result: Result<CompositeAttribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");

        let input = "#![meta]"; // ! not allowed
        let result: Result<CompositeAttribute, Error> = parse_attribute_from_str(input);
        assert!(result.is_err(), "Parsed an invalid attribute successfully");
    }
}

/// Tests for [AttributeModifier](crate::attributes::AttributeModifier)
mod field_attribute {
    use super::*;
    #[test]
    fn parse_valid_field_attribute_many() {
        let input ="#>[one]* #[a] #>[two]";
        let result : Result<Vec<FieldAttribute>, Error> = parse_attribute_from_str_many(input);
        if let Ok(attrs) = result {
            assert_eq!(attrs.len(), 3);
            // Additional checks for each attribute
        }
    }

    #[test]
    fn parse_invalid_field_attribute() {
        // Examples of invalid nested attribute inputs
        let inputs = vec![
            "#>[]", // Missing inner content
            "#>meta", // No Brackets
            "#[nested(Debug)]*", // Modifier isn't allowed on # (only on #>)
        ];

        for input in inputs {
            let result: Result<FieldAttribute, Error> = parse_attribute_from_str(input);
            assert!(result.is_err(), "Parsed an invalid nested attribute successfully");
        }
    }
    
    #[test]
    fn parse_correct_map_confirm() {
        let input = "#[attr]";
        let attr: FieldAttribute = parse_attribute_from_str(input).unwrap();
        assert!(matches!(attr, FieldAttribute::Field(_))); // Make sure its type Field

        let input = "#>[attr]*";
        let attr: FieldAttribute = parse_attribute_from_str(input).unwrap();
        assert!(matches!(attr, FieldAttribute::Nested(_))); // Make sure it is type Nested
    }
}

/// Tests for [AttributeModifier](crate::attributes::AttributeModifier)
mod attribute_modifier {
    use super::*;

    #[test]
    fn parse_star_modifier() {
        let input = "*";
        let parsed = parse_str::<AttributeModifier>(input);

        assert!(matches!(parsed, Ok(AttributeModifier::Star(_))));
    }

    #[test]
    fn parse_slash_modifier() {
        let input = "/";
        let parsed = parse_str::<AttributeModifier>(input);

        assert!(matches!(parsed, Ok(AttributeModifier::Slash(_))));
    }

    #[test]
    fn parse_minus_modifier() {
        let input = "-";
        let parsed = parse_str::<AttributeModifier>(input);

        assert!(matches!(parsed, Ok(AttributeModifier::Minus(_))));
    }

    #[test]
    fn parse_should_fail() {
        let input = "&";
        let parsed = parse_str::<AttributeModifier>(input);

        assert!(parsed.is_err(), "Parsed token when should fail")
    }
}

/// Helper method to make parsing an attribute easy because its wierd
fn parse_attribute_from_str<A: ParseAttribute>(input: &str) -> Result<A, Error> {
    struct AttributeWrapper<A: ParseAttribute> {
        attribute: A
    }

    impl<A: ParseAttribute> Parse for AttributeWrapper<A> {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let a = input.call(A::parse_single_outer)?;
            Ok(AttributeWrapper {
                attribute: a
            })
        }
    }

    let wrapper = parse_str::<AttributeWrapper<A>>(input)?;
    Ok(wrapper.attribute)
}

fn parse_attribute_from_str_many<A: ParseAttribute>(input: &str) -> Result<Vec<A>,Error> {
    struct AttributeWrapper<A: ParseAttribute> {
        attribute: Vec<A>
    }

    impl<A: ParseAttribute> Parse for AttributeWrapper<A> {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let a = input.call(A::parse_outer)?;
            Ok(AttributeWrapper {
                attribute: a
            })
        }
    }

    let wrapper = parse_str::<AttributeWrapper<A>>(input)?;
    Ok(wrapper.attribute)
}
