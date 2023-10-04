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
fn test_nested() {
    nest! {
        struct Parent {
            f: (),
            child: struct Child { }
        }
    }

    let _ = Parent { f: (), child: Child {} };
}

#[test]
fn test_generics() {
    nest! {
        struct Struct<'a, T> {
            s: &'a str,
            t: T
        }
    }

    let _ = Struct { s: "", t: () };
}

#[test]
fn test_nested_generics() {
    nest! {
        struct Parent<'p, P> {
            child ::<'p, P> : struct Child<'c, C> {
                s: &'c str,
                f: C
            }
        }
    }

    let _ = Parent { child: Child { s: "", f: () } };
}

#[test]
fn test_complex_types() {
    nest! {
        struct Struct {
            payload: Box<Result<String, Option<bool>>>
        }

    }

    let _ = Struct { payload: Box::new(Ok("Test".to_string())) };
}


#[test]
fn test_basic_attrs() {
    nest! {
        #[derive(Clone)]
        struct CloneMe { }
    }
    let x = CloneMe{};
    let c = x.clone();
}

#[test]
fn test_attrs() {
    nest! {
        struct NoClone {
            c: #[derive(Clone)] struct YesClone { }
        }
    }

    let _ = NoClone { c: YesClone { } };
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

