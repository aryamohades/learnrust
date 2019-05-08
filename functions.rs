#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // function argument type is explicit
    // function return type is explicit
    // implicit return (no `return` keyword or semicolon)
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn takes_ownership(s: String) {
        // do something with `s`
    }

    fn does_not_take_ownership(s: &String) {
        // do something with `s`
    }

    let s = String::from("hello");
    takes_ownership(s);
    // passing a non reference variable into function will move the variable
    // does not compile
    // println!("{}", s);

    // pass reference to function to avoid moving
    let mut s = String::from("hello");
    does_not_take_ownership(&s);
    s.push_str(", world");

    fn primitive(x: i32) {
        // do something with `x`
    }

    // pass primitives into functions without moving because they are copied
    let x = 5;
    primitive(x);
    let sum = x + 10;
}