#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // basic struct
    struct Point {
        x: i32,
        y: i32,
    }

    // create an instance of a struct
    let origin: Point = Point {
        x: 0,
        y: 0
    };

    // create a struct defined within a module to demonstrate public/private struct fields
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

    // create an instance of a struct using the static constructor method `new`
    let mut john = person::Person::new("John", "Doe");

    // private struct field cannot be directly accessed
    // does not compile
    // john.sign_in_count;

    // access public struct fields
    assert_eq!(john.first_name, "John");
    assert_eq!(john.full_name, "John Doe");

    // call struct instance method
    john.sign_in();
    assert_eq!(john.sign_in_count(), 1);
    
    // call struct static method
    assert_eq!(person::Person::get_description(), "A person");

    // a struct with unnamed fields, known as a `tuple struct’
    struct Point2(i32, i32);

    // create an instance of a tuple struct
    let point = Point2(5, 10);

    // access fields of tuple struct (same as accessing fields of tuple)
    assert_eq!(point.0, 5);
    assert_eq!(point.1, 10);

    // why would we use a tuple struct over a tuple?
    // with tuple structs, we can create a custom type
    // for more clarity about what the code is doing.
    // for example, we can represent an rgb color with
    // a regular tuple:
    //
    // let color: (i32, i32, i32) = (255, 255, 255);
    //
    // but we could already represent position in the same way:
    //
    // let position: (i32, i32, i32) = (255, 255, 255);
    //
    // with this setup, we can't differentiate between position
    // and color and a function that accepts a tuple with three
    // i32 would accept both color and position tuples.
    // if we used two separate tuple structs to represent Color and Position,
    // then we can create a function that only accepts a `Color` or only accepts
    // a `Position` as an argument
    struct Color(i32, i32, i32);
    struct Position(i32, i32, i32);

    struct Foo<T> {
        bar: T
    }

    // traits (known as interfaces or typeclasses in other languages)
    trait GetBar<T> {
        fn get_bar(self) -> T;
    }

    // implement the trait GetBar for the Foo struct
    impl<T> GetBar<T> for Foo<T> {
        fn get_bar(self) -> T {
            self.bar
        }
    }

    // call struct trait method
    let a_foo = Foo { bar: 1 };
    assert_eq!(a_foo.get_bar(), 1);

    // create another instance of Foo with a different type,
    // demonstrating basic usage of generics with `T`
    let a_foo = Foo { bar: "hello" };
    assert_eq!(a_foo.get_bar(), "hello");
}