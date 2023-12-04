#![cfg(test)]

use nestify2::nest;

#[test]
fn please_work() {
    nest!(
        struct Hello {
            t: ~~~~,
        }
    );
    let _ = Works {};
}

