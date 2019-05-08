#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // Create a heap-allocated string
    let s = String::from("hello");

    // Create a string in a different way
    let s = "hello".to_string();

    // String literal
    let s = "hello";

    let mut s = String::from("hello");

    // concatenate char
    s.push(',');
    assert_eq!(s, "hello,");

    // concatenate string
    s.push_str(" world");
    assert_eq!(s, "hello, world");

    // slice - immutable view into a string
    let slice = &s;

     // Iterate over string characters
    for c in s.chars() { 
        // do something with `c`
    }

    // Iterate over string characters with index
    for (i, c) in s.chars().enumerate() {
        // do something with character `c` and index `i`
    }

    let bytes = s.as_bytes();
    assert_eq!(bytes, [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100]);

    // iterate over bytes of string
    for &b in s.as_bytes().iter() {
        // do something with `b`
    }

    // Function that returns a string slice representing first word in string
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    assert_eq!(first_word("hello world"), "hello");
}