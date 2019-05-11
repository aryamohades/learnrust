use std::fmt::Display;

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // define a trait `Summary` with a single method `summarize`
    // no implementation of `summarize` is given here because each type
    // implementing `Summary` must provide its own custom implementation
    trait Summary {
        fn summarize(&self) -> String;
    }

    // `Debug` trait with a single method `debug`
    trait Debug {
        fn debug(&self);
    }

    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }

    // example of implementing trait `Summary` for type `NewsArticle`
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    let article = NewsArticle {
        headline: "foo".to_string(),
        location: "nowhere".to_string(),
        author: "nobody".to_string(),
        content: "nothing".to_string()
    };

    // example of calling trait method on instance of `NewsArticle`
    article.summarize();

    // trait with default implementation
    trait SummaryWithDefault {
        fn summarize_with_default(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // use trait with default implementation for `NewsArticle` with empty impl block
    impl SummaryWithDefault for NewsArticle {}

    // use trait as function argument
    // function that accepts a type that implements `Summary` trait
    fn notify_1(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // longer syntax version of notify, known as a 'trait bound'
    fn notify_2<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait bound is useful in more complex situations, example
    // if you need two different parameters that implement `Summary` trait
    // and they both have to be the same type
    fn notify_3<T: Summary>(item1: T, item2: T) {}

    // multiple traits in a trait bound using `+`
    fn notify_4(item: impl Summary + Display) {}

    // multiple traits on a generic type in a trait bound using `+`
    fn notify_5<T: Summary + Display>(item: T) {}

    // using lots of trait bounds can be messy
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

    // alternate syntax using `where` to make multiple trait bounds easier to understand
    fn some_function_cleaner<T, U>(t: T, u: U)
        where T: Display + Clone, U: Clone + Debug {}

    // return a type that implements a trait
    // rather than specify a concrete return type, we just say
    // return some type that implements the `Summary` trait
    fn returns_summarizable() -> impl Summary {
        NewsArticle {
            headline: "foo".to_string(),
            location: "nowhere".to_string(),
            author: "nobody".to_string(),
            content: "nothing".to_string()
        }
    }

    // now, we can implement a `largest` function that takes in a list of
    // generic type items and returns the largest item, whatever largest means.
    // type `T` must implement the `PartialOrd` and `Copy` traits
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];
    assert_eq!(largest(&numbers), 100);
    assert_eq!(largest(&chars), 'y');

    // use trait bounds to conditionally implement methods
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // only implement the `cmp_display` method if the type `T` passed to `Pair`
    // implements the `Display` and `PartialOrd` traits
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // use traits to allow functions to accept either String or &str
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