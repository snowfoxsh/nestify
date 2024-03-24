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

enum EnumVariants {
    Unit,
    Tuple(i32, TupleNested),
    Struct {
        f1: i32
    },
    // DiscriminantVariant = 1,
}
struct TupleNested;
