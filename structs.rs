#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Basic struct
    struct Point {
        x: i32,
        y: i32,
    }

    // Create an instance of a struct
    let origin: Point = Point {
        x: 0,
        y: 0
    };

    // Create a struct defined within a module to demonstrate public/private struct fields
    mod person {
        pub struct Person {
            pub first_name: String,
            pub last_name: String,
            pub full_name: String,
            sign_in_count: u32
        }

        impl Person {
            // constructor method
            pub fn new(first_name: &str, last_name: &str) -> Person {
                Person {
                    first_name: String::from(first_name),
                    last_name: String::from(last_name),
                    full_name: format!("{} {}", first_name, last_name),
                    sign_in_count: 0
                }
            }

            // a struct method that takes &self as parameter is an instance method
            pub fn sign_in(&mut self) -> u32 {
                self.sign_in_count += 1;
                self.sign_in_count
            }

            pub fn sign_in_count(&self) -> u32 {
                self.sign_in_count
            }

            // static struct method
            pub fn get_description() -> String {
                "A person".to_string()
            }
        }
    }

    // Create an instance of a struct
    let mut john = person::Person::new("John", "Doe");

    // Cannot access a private struct field directly
    // Does not compile:
    // john.sign_in_count;

    // Access struct fields
    assert_eq!(john.first_name, "John");
    assert_eq!(john.full_name, "John Doe");

    // Call struct method
    john.sign_in();
    assert_eq!(john.sign_in_count(), 1);
    
    // Call a static method of struct
    assert_eq!(person::Person::get_description(), "A person");

    // A struct with unnamed fields, called a ‘tuple struct’
    struct Point2(i32, i32);

    // Create an instance of a tuple struct
    let point = Point2(5, 10);

    // Access fields of tuple struct
    assert_eq!(point.0, 5);
    assert_eq!(point.1, 10);

    // Why would we use a tuple struct over a tuple?
    // Because we can create our own type and use that
    // to validate function arguments and the like.
    // We can have two tuple structs that have the
    // same types but represent two different objects e.g.
    struct Color(i32, i32, i32);
    struct Position(i32, i32, i32);

    struct Foo<T> {
        bar: T
    }

    // Traits (known as interfaces or typeclasses in other languages)
    trait GetBar<T> {
        fn get_bar(self) -> T;
    }

    // Implement the trait DoAThing for the Foo struct
    impl<T> GetBar<T> for Foo<T> {
        fn get_bar(self) -> T {
            self.bar
        }
    }

    // Call struct trait method
    let a_foo = Foo { bar: 1 };
    assert_eq!(a_foo.get_bar(), 1);

    // This also demonstrates basic usage of generic types with `T`
    // Create another instance of Foo with a different type
    let a_foo = Foo { bar: "hello" };
    assert_eq!(a_foo.get_bar(), "hello");
}