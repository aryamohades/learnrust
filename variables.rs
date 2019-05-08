#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    // Constant variable
    const MAX_POINTS: u32 = 100_000;

    // Immutable variable with explicit type
    let num: i32 = 1;
    let num2: u8 = 10;
    let num3: f64 = 1.3;

    // Cannot mutate immutable variable
    // Does not compile:
    // num += 1;

    // Variable with inferred type
    let num = 1;

    // Mutable variable
    let mut num = 5;
    num += 1;

    // Shadow variables to change type without changing name
    let spaces = "   ";
    let spaces = spaces.len();

    // Ownership //

    // Create a reference to an i32
    let mut num: Box<i32> = Box::new(5);
    *num = 6;
    assert_eq!(*num, 6);

    // Transfer ownership (num is 'moved' to another_num)
    let mut another_num = num;
    *another_num = 10;
    assert_eq!(*another_num, 10);

    // `num` is no longer valid because it was moved to `another_num`
    // Does not compile:
    // *num = 5;

    // References/Borrowing //

    let num = 5;

    // Borrow num with an immutable reference
    let ref_num = &num;

    // Cannot mutate a variable while it is borrowed
    // Does not compile:
    // num = 6;

    // Can still use num because the reference to it is immutable
    let sum = num + 5;
    assert_eq!(sum, 10);
    
    let mut num = 10;

    // Create a mutable reference to num
    let ref_num = &mut num;

    // If we have a mutable reference to num, num cannot be accessed
    // Does not compile:
    // let sum = num + 10;

    // Cannot have two mutable references to the same variable
    let mut num = 10;
    let ref_num_1 = &mut num;
    // Does not compile:
    // let ref_num_2 = &mut num;

    // Cannot have a mutable and immutable ref to the same variable
    let mut num = 10;
    let ref_num_1 = &num;
    // Does not compile:
    // let ref_num_2 = &mut num;
}