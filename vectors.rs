#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
fn main() {
    // Create a fixed size array
    let arr = [1, 2, 3];

    // Access array element
    assert_eq!(arr[0], 1);

    // Create immutable vector
    let nums: Vec<i32> = Vec::new();

    // Does not compile:
    // nums.push(1);

    // Create and initialize vector with vec! macro
    let nums = vec![1, 2, 3];

    // Mutable vector
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    assert_eq!(nums, [1, 2, 3, 4]);

    // A fixed size vector of four i32
    let four_i32: [i32; 4] = [1, 2, 3, 4];

    // Vector with fixed length and filled values
    let zero_vec = vec![0; 5];
    assert_eq!(zero_vec, [0, 0, 0, 0, 0]);

    // Access first element of vector
    assert_eq!(nums[0], 1);

    // Safe access using `get`, returns an Option<T>
    assert_eq!(*nums.get(2).unwrap(), 3);

    // Access element that does not exist
    match nums.get(10) {
        Some(num) => (),
        None => ()
    }

    // Loop through vector
    for num in &nums {
        // do something with `num`
    }

    // Loop through vector with indexes
    for (index, num) in nums.iter().enumerate() {
        // do something with `index` and `num`
    }

    // Double each element in vector in place
    for num in &mut nums {
        *num *= 2;
    }
    assert_eq!(nums, [2, 4, 6, 8]);

    // Double each element in a functional way using `map` (creates new vector)
    let doubles: Vec<i32> = nums.into_iter().map(|x| x * 2).collect();
    assert_eq!(doubles, [4, 8, 12, 16]);

    let nums = vec![1, 2, 3];
    // slice - immutable view into vector
    let slice = &nums;
}