#![allow(dead_code)]
#![allow(unused_variables)]
#![cfg(test)]

use nestify::nest;

// todo: write more tests

#[test]
fn test_empty_struct() {
    nest! {
        struct Struct {}
    }

    let _ = Struct {};
}

#[test]
fn test_rec_attrs() {
    nest! {
        #[derive(Debug)]*
        struct GrandParent {
            parent: struct Parent {
                child: struct Child {
                    payload: ()
                }
            }
        }
    }

    let test = GrandParent { parent: Parent { child: Child { payload: () } } };
    assert_eq!(
        "GrandParent { parent: Parent { child: Child { payload: () } } }",
        format!("{:?}", test)
    );
}

