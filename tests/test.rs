#![cfg(test)]

use nestify2::nest;

#[test]
fn es() {}

#[test]
fn please_work() {
    let s = 32;
    nest!(
        #[derive(Debug)]
        struct Hello {

            pub o: 
            struct Another {
                s: #[derive(Debug)] struct A {
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
        #[derive(Debug)]*
        enum Outside {
            Empty,
            Tuple(Option<i32>, i32, enum Nested1 { }),
            Named {
                a: (),
                b: Option<i32>,
                c: enum Nested2 {
                    Tuple2(enum Nested3 {})
                }
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
}

#[test]
fn test_semis() {

}

#[derive(Default)]
enum AnEnum {
    One = 1,
    #[default]
    Default,
    Two,
}