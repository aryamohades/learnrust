#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // `largest` is generic over some type T,
    // accepts one argument of type T, and returns the value
    fn largest<T>(val: T) -> T {
        val
    }

    // struct to hold x and y coordinate values of any type
    struct Point<T> {
        x: T,
        y: T
    }

    // struct method that uses a generic type
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    // implement function only for Point<f32>
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // cannot mix types with only a single generic type
    // does not compile
    // let wont_work = Point { x: 5, y: 4.0 };

    // a struct that accepts mixed types
    struct PointMixed<T, U> {
        x: T,
        y: U,
    }

    let both_integer = PointMixed { x: 5, y: 10 };
    let both_float = PointMixed { x: 1.0, y: 4.0 };
    let integer_and_float = PointMixed { x: 5, y: 4.0 };
}