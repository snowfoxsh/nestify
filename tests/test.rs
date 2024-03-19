#![cfg(test)]

use nestify2::nest;

#[test]
fn please_work() {
    nest!(
        struct Hello {
            pub o: struct Another {
                #[derive(Debug)]
                s: struct A {
                    a: struct B {

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
