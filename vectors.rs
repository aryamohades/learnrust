#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // create an immutable i32 array of size 3, initialized with zeroes
    let arr: [i32; 3] = [0; 3];

    // create a fixed size array
    let arr = [1, 2, 3];

    // iterate over an array
    for i in arr.iter() {
        // do something with `i`
    }

    // the array itself is not iterable
    // does not compile
    // for i in arr {}

    // if the array if size 32 or less, you can iterate using the reference
    for i in &arr {
        // do something with `i`
    }

    // array of size 33 or more does not implement IntoIterator
    // does not compile
    // let arr = [0; 33];
    // for i in &arr {
    //     // do something with `i`
    // }

    // access array element
    assert_eq!(arr[0], 1);

    // create immutable vector
    let nums: Vec<i32> = Vec::new();

    // does not compile
    // nums.push(1);

    // create and initialize vector with vec! macro
    let nums = vec![1, 2, 3];

    // mutable vector
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    assert_eq!(nums, [1, 2, 3, 4]);

    // a fixed size vector of four i32
    let four_i32: [i32; 4] = [1, 2, 3, 4];

    // vector with fixed length and filled values
    let zero_vec = vec![0; 5];
    assert_eq!(zero_vec, [0, 0, 0, 0, 0]);

    // access first element of vector
    assert_eq!(nums[0], 1);

    // safe access using `get`, returns an Option<T>
    assert_eq!(*nums.get(2).unwrap(), 3);

    // access element that does not exist
    match nums.get(10) {
        Some(num) => (),
        None => ()
    }

    // loop through vector
    for num in &nums {
        // do something with `num`
    }

    // loop through vector with indexes
    for (index, num) in nums.iter().enumerate() {
        // do something with `index` and `num`
    }

    // double each element in vector in place
    for num in &mut nums {
        *num *= 2;
    }
    assert_eq!(nums, [2, 4, 6, 8]);

    // double each element in a functional way using `map` (creates new vector)
    let doubles: Vec<i32> = nums.into_iter().map(|x| x * 2).collect();
    assert_eq!(doubles, [4, 8, 12, 16]);

    let nums = vec![1, 2, 3];
    // slice - immutable view into vector
    let slice = &nums;
}