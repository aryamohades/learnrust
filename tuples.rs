#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // tuple with explicit typing
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // tuple with inferred typing
    let x = (1, "hello", 3.4);

    // destructure assignment
    let (a, b, c) = x;
    assert_eq!(a, 1);
    assert_eq!(b, "hello");
    assert_eq!(c, 3.4);

    // access tuple fields
    assert_eq!(x.0, 1);
    assert_eq!(x.1, "hello");
    assert_eq!(x.2, 3.4);
}