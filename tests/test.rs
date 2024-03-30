#![cfg(test)]

#![allow(dead_code, unused_variables)]

use nestify::nest;

#[test]
fn es() {}

#[test]
fn please_work() {
    let s = 32;
    nest!(
        struct Hello {
            pub o:
            struct Another {
                s: struct A {
                    a: struct B {
                        asdfs: struct BB {

                        },
                    },
                    c: i32,
                    d: struct D();,
                    asd: Option<i32>,
                },
            },
            pub unmaed: struct Unamed;,
        }
    );
}

#[test]
fn enum_test() {
    nest! {
        enum Outside {
            Empty,
            Tuple(Option<i32>, i32, enum Nested1 { }),
            #>[derive(Debug)]*
            Named {
                a: (),
                b: Option<i32>,
                c: enum Nested2 {
                    Tuple2(enum Nested3 {})
                },
                d: struct Another {}
            }
        }
    }
}

#[test]
fn attribute_test() {
    nest! {
        #[derive(Default)]
        enum AnEnum {
            #[default]
            One = 1,
            Default,
            Two,
        }
    }
    
    // nest! {
    //     struct Outside {
    //         f : struct Inside<G> {
    //             gf: G
    //         },
    //     }
    // }
    
    let v = vec![1, 2, 3];
    
    let v2 = v.into_iter().collect::<Vec<i32>>();
    
    nest! {
        struct A<T>(T) where T: Clone
    }
}

#[test]
fn test_semis() {
    // nest! {
    //     struct Two(::<i32> struct One<T>(T))
    // }
    
    nest! {
        struct Outside (
            enum E<T> {Se(T)} ||<i32>,
        )
    }
}

struct One1<T> (T)
;
struct Two1(
    One1<i32>)
;

#[derive(Default)]
enum AnEnum {
    One = 1,
    #[default]
    Default,
    Two,
}


struct NeedsLife<'a> {
    string: &'a str
}
#[test]
fn test_fish_errors() {
    nest! {
        struct Base(
            struct NoError<T> {
                tf: T
            } ||<i32>)
    }
}



#[test]
fn augmented_types() {
    {
        nest! {
            struct Foo {
                field: Option<struct Bar {
                    foo: u32
                }>,
            }
        };

        let foo = Foo { field: Some(Bar { foo: 42 })};
    }

    {
        nest! {
            struct Foo(
                Option<pub struct Bar {
                    foo: u32
                }>,
            )
        };

        let foo = Foo (Some(Bar { foo: 42 }));
    }

    {
        nest! {
            enum Foo {
                Variant {
                    field: Option<#[derive(Debug)] struct Bar {
                        foo: u32
                    }>,
                }
                
            }
        };

        let foo = Foo::Variant { field: Some(Bar { foo: 42 })};
    }

    {
        nest! {
            #[derive(Debug)]*
            enum Foo {
                Variant(
                    Option<struct Bar {
                        foo: u32
                    }>,
                )
            }
        };

        let foo = Foo::Variant (Some(Bar { foo: 42 }));
        println!("{foo:?}");
    }

    {
        nest! {
            struct Foo {
                field: Option<struct Bar<T>(T) ||<u32>>,
            }
        };

        let foo = Foo { field: Some(Bar(42)) };
        // check it is actually a generic struct
        let bar: Bar<String> = Bar("test".to_string());
    }
    
}