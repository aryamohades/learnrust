#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    struct Person {
        name: String,
    }

    impl Person {
        // use Into trait to accept a String or &str as argument
        fn new<S: Into<String>>(name: S) -> Person {
            Person { name: name.into() }
        }
    }

    let person = Person::new("bob");
    let person = Person::new("bob".to_string());
}