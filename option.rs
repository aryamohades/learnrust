#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // accept an Option and add 1 to value if exists, return
    // None otherwise
    fn add_one_or_none(val: Option<i32>) -> Option<i32> {
        val.map(|x| x + 1)
    }

    assert_eq!(add_one_or_none(Some(4)), Some(5));
    assert_eq!(add_one_or_none(None), None);

    // accept an Option and add 1 to value if exists, return
    // 0 otherwise
    fn add_one_or_zero(val: Option<i32>) -> i32 {
        val.map_or(0, |x| x + 1)
    }

    assert_eq!(add_one_or_zero(Some(10)), 11);
    assert_eq!(add_one_or_zero(None), 0);

    // use match to do something based on existence of value
    let result = match Some(42) {
        Some(i) => i + 10,
        None => 0
    };

    assert_eq!(result, 52);

    let val: Option<i32> = None;
    let result = match val {
        Some(i) => i + 10,
        None => 0
    };

    assert_eq!(result, 0);
}