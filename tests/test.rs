#![cfg(test)]

use nestify2::nest;

#[test]
fn please_work() {
    // bug: there is a bug where a comma is not parsed correctly
    // it should be in the parsing code

    // bug: there is a bug where a structure cannot have no field
    // like `struct A { }`
    nest!(
        struct Hello {
            o: struct Another {
                s: struct A{ s: i32 },
            },
        }
    );
}




