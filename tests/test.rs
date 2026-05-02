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

#[test]
fn partial_attribute_removal() {
    use std::fmt::{Debug};

    // macros based off of https://github.com/nvzqz/static-assertions
    macro_rules! does_impl_one {
        ($ty:ty: $($trait_bound:tt)+) => {{
            use core::marker::PhantomData;

            trait DoesntImpl {
                const DOES_IMPL: bool = false;
            }

            impl<T: ?Sized> DoesntImpl for T {}

            struct Wrapper<T: ?Sized>(PhantomData<T>);

            impl<T: ?Sized + $($trait_bound)+> Wrapper<T> {
                const DOES_IMPL: bool = true;
            }

            <Wrapper<$ty>>::DOES_IMPL
        }};
    }

    macro_rules! assert_impl_all {
        ($ty:ty: $($trait_bound:path),+ $(,)?) => {{
            $(
                assert_eq!(
                    does_impl_one!($ty: $trait_bound),
                    true,
                    "expected `{}` to implement `{}`",
                    stringify!($ty),
                    stringify!($trait_bound),
                );
            )+
        }};
    }

    macro_rules! assert_impl_not_all {
        ($ty:ty: $($trait_bound:path),+ $(,)?) => {{
            let actual = true $(&& does_impl_one!($ty: $trait_bound))+;

            assert_eq!(
                actual,
                false,
                "expected `{}` to not implement all of `{}`",
                stringify!($ty),
                stringify!($($trait_bound),+),
            );
        }};
    }

    macro_rules! assert_impl_none {
        ($ty:ty: $($trait_bound:path),+ $(,)?) => {{
            $(
                assert_impl_not_all!($ty: $trait_bound);
            )+
        }};
    }

    {
        // test macros
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Full;

        struct Plain;

        assert_impl_all!(Full: Debug, Clone, PartialEq, Eq);

        assert_impl_none!(Plain: Debug, Clone, PartialEq, Eq);
    }

    {
        nest! {
        #[derive(Debug, Clone)]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[derive(Debug, Clone)]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone);
        assert_impl_none!(Bar: Debug, Clone);
    }

    {
        nest! {
            #[derive(Debug, Clone)]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[derive(Debug)]-
                    struct Bar {
                        child: std::marker::PhantomData<
                            struct Baz {
                                foo: u32,
                            }
                        >,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone);

        assert_impl_all!(Bar: Clone);
        assert_impl_none!(Bar: Debug);

        // - only affects the current item, not descendants
        assert_impl_all!(Baz: Debug, Clone);
    }

    {
        nest! {
            #[derive(Debug, Clone)]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[derive(Debug)]/
                    struct Bar {
                        child: std::marker::PhantomData<
                            struct Baz {
                                foo: u32,
                            }
                        >,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone);

        assert_impl_all!(Bar: Clone);
        assert_impl_none!(Bar: Debug);

        // / affects descendants too
        assert_impl_all!(Baz: Clone);
        assert_impl_none!(Baz: Debug);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug, Eq, PartialEq), derive(Clone))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(derive(Debug, Eq, Clone))]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone, PartialEq, Eq);

        // expected #[cfg_attr(all(), derive(PartialEq))] leftover
        assert_impl_all!(Bar: PartialEq);
        assert_impl_none!(Bar: Debug, Clone, Eq);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug))]*
            #[cfg_attr(any(), derive(Clone))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(derive(Clone))]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug);
        assert_impl_none!(Foo: Clone);

        // if this expansion leaves #[cfg_attr(any())], this test should fail before assertions even run
        assert_impl_all!(Bar: Debug);
        assert_impl_none!(Bar: Clone);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug, Eq, PartialEq))]*
            #[cfg_attr(all(), derive(Clone))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(derive(Debug, Eq, Clone))]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone, PartialEq, Eq);

        // expected #[cfg_attr(all(), derive(PartialEq))] leftover
        assert_impl_all!(Bar: PartialEq);
        assert_impl_none!(Bar: Debug, Clone, Eq);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug))]*
            #[cfg_attr(any(), derive(Debug))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(derive(Debug))]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug);

        // the nearest matching cfg_attr is the inactive any() one. should leave the all() one intact
        assert_impl_all!(Bar: Debug);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug, Eq, PartialEq), derive(Clone))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(derive(Debug, Clone))]/
                    struct Bar {
                        child: std::marker::PhantomData<
                            struct Baz {
                                foo: u32,
                            }
                        >,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug, Clone, PartialEq, Eq);

        // expected #[cfg_attr(all(), derive(Eq, PartialEq))] leftover for Bar and Baz
        assert_impl_all!(Bar: PartialEq, Eq);
        assert_impl_none!(Bar: Debug, Clone);

        assert_impl_all!(Baz: PartialEq, Eq);
        assert_impl_none!(Baz: Debug, Clone);
    }

    {
        nest! {
            #[cfg_attr(all(), derive(Debug))]*
            #[cfg_attr(any(), derive(Clone))]*
            struct Foo {
                field: std::marker::PhantomData<
                    #[cfg_attr(any(), derive(Clone))]-
                    struct Bar {
                        foo: u32,
                    }
                >,
            }
        }

        assert_impl_all!(Foo: Debug);
        assert_impl_none!(Foo: Clone);

        assert_impl_all!(Bar: Debug);
        assert_impl_none!(Bar: Clone);
    }
}