#![cfg(test)]

use nestify2::nest;

#[test]
fn please_work() {
    nest!(
        struct Hello {
            pub o: struct Another {
                s: struct A {
                    a: struct B {
                        asdfs: struct BB {

                        },
                    },
                    c: i32,
                    d: struct D {

                    },
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
            Named {
                a: i32,
                b: Option<i32>,
                c: enum Nested2 {
                    Tuple2(enum Nested3 {})
                }
            }
        }
    }
}
