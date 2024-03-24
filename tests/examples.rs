use nestify::nest;



#[test]
fn quick_examples_one() {
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
fn quick_examples_two() {
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

