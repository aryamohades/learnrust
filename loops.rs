#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // for loop
    let nums = [1, 2, 3];
    for num in nums.iter() {
        // do something with `num`
    }

    // range loop (non-inclusive)
    for i in 0..10 {
        // do something with `num`
    }

    // while loop
    while 1 == 1 {
        // do something
        // break out of loop
        break
    }

    // Infinite loop
    loop {
        // do something
        // break out of loop
        break
    }

    let mut counter = 0;

    // Break out of loop with an expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}