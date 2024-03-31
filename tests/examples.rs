//! Contains examples from the guide in README.md

#![allow(dead_code, unused_variables)]

use nestify::nest;


mod quick_examples {
    use super::*;

    #[test]
    fn simple_nested_structures() {
        nest! {
            struct UserProfile {
                name: String,
                address: struct Address {
                    street: String,
                    city: String,
                },
                preferences: struct Preferences {
                    newsletter: bool,
                },
            }
        }
    }

    #[test]
    fn simple_nested_enums() {
        nest! {
            struct Task {
                id: i32,
                description: String,
                status: enum Status {
                    Pending,
                    InProgress,
                    Completed,
                },
            }
        }
    }
}

mod supported_definitions {
    use super::*;

    #[test]
    fn field_structs() {
        nest! {
            struct Named {
                f: struct Nested {}
            }
        }
    }

    #[test]
    fn tuple_structs() {
        nest! {
            struct Unnamed(struct Nested())
        }
    }

    #[test]
    fn unit_structs() {
        nest! {
            struct Unit {
                unit: struct UnitStruct
            }
        }
    }

    #[test]
    fn enum_variants() {
        nest! {
            enum EnumVariants {
                Unit,
                Tuple(i32, struct TupleNested),
                Struct {
                    f1: i32,

                },
                // DiscriminantVariant = 1,
            }
        }
    }
}

mod generics {
    use super::*;

    #[test]
    fn generic() {
        nest! {
            struct Example<'a, T> {
                s: &'a str,
                t: T
            }
        }
    }

    fn nested_generics() {
        nest! {
            struct Parent<'a> {
                child : struct Child<'c, C> {
                    s: &'c str,
                    f: C
                } ||<'a, i32>
            }
        }
    }
}

mod field_attributes {
    use super::*;

    #[test]
    fn enum_variants() {
        nest! {
            enum MyEnum {
                #>[derive(Debug)]
                Variant {
                    // #[derive(Debug)
                    one: struct One,
                    // #[derive(Debug)
                    two: struct Two
                }
            }
        }
    }
}

mod visibility {
    use super::*;

    #[test]
    fn named_fields() {
        nest! {
            pub struct One {
                pub two: pub struct Two
                //|      ^^^ visibility applied to definition (2)
                //|> visibility applied to field (1)
            }
        }
    }

    #[test]
    fn unnamed_fields() {
        nest! {
            pub struct One(pub struct Two)
            //             ^^^ visibility applied to both field and struct
        }
    }

    #[test]
    fn enum_variants() {
        nest! {
            pub enum One {
                Two(pub struct Two)
            }
        }
    }
}

mod generic_definitions {
    use super::*;
    fn vec_example() {
        nest! {
            struct One(Vec<struct Two { field: i32 }>);
        }
    }
    
    fn option_example() {
        nest! {
            struct AppConfig(Option<struct DatabaseConfig { url: String }>);
        }
    }
}

// enum EnumVariants {
//     Unit,
//     Tuple(i32, TupleNested),
//     Struct {
//         f1: i32
//     },
//     // DiscriminantVariant = 1,
// }
// struct TupleNested;
