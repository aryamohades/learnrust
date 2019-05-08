#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Basic pattern matching
    let foo: Option<i32> = Some(1);
    match foo {
        Some(n) => println!("it’s an i32: {}", n),
        None => println!("it’s nothing!"),
    }

    // Advanced pattern matching
    struct FooBar { x: i32, y: Option<i32> }
    let bar = FooBar { x: 15, y: Some(32) };

    match bar {
        FooBar { x: 0, y: Some(0) } =>
            println!("the numbers are zero!"),
        FooBar { x: n, y: Some(m) } if n == m =>
            println!("the numbers are the same"),
        FooBar { x: n, y: Some(m) } =>
            println!("different numbers: {} {}", n, m),
        FooBar { x: _, y: None } =>
            println!("the second number is Nothing"),
    }

    enum Number {
        One,
        Two,
        Three,
        Four,
        Five
    }

    let number = Number::Three;

    // Use `_` to catch all unhandled cases
    match number {
        Number::One => println!("One!"),
        Number::Two => println!("Two!"),
        _ => println!("not One or Two")
    }

    // If we only want to match on a single value, we can use `if let` instead to be more concise
    if let Number::Three = number {
        println!("Three!");
    }
}