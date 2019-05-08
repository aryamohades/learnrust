#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
fn main() {
    // constant variable
    const MAX_POINTS: u32 = 100_000;

    // immutable variable with explicit type
    let num: i32 = 1;
    let num2: u8 = 10;
    let num3: f64 = 1.3;

    // cannot mutate immutable variable
    // does not compile
    // num += 1;

    // variable with inferred type
    let num = 1;

    // mutable variable
    let mut num = 5;
    num += 1;

    // shadow variables to change type without changing name
    let spaces = "   ";
    let spaces = spaces.len();

    // ownership //

    // create a reference to an i32
    let mut num: Box<i32> = Box::new(5);
    *num = 6;
    assert_eq!(*num, 6);

    // transfer ownership (num is 'moved' to another_num)
    let mut another_num = num;
    *another_num = 10;
    assert_eq!(*another_num, 10);

    // `num` is no longer valid because it was moved to `another_num`
    // does not compile
    // *num = 5;

    // references/borrowing //

    let num = 5;

    // borrow num with an immutable reference
    let ref_num = &num;

    // cannot mutate a variable while it is borrowed
    // does not compile
    // num = 6;

    // can still use num because the reference to it is immutable
    let sum = num + 5;
    assert_eq!(sum, 10);
    
    let mut num = 10;

    // create a mutable reference to num
    let ref_num = &mut num;

    // if we have a mutable reference to num, num cannot be accessed
    // does not compile
    // let sum = num + 10;

    // cannot have two mutable references to the same variable
    let mut num = 10;
    let ref_num_1 = &mut num;
    // does not compile
    // let ref_num_2 = &mut num;

    // cannot have a mutable and immutable ref to the same variable
    let mut num = 10;
    let ref_num_1 = &num;
    // does not compile
    // let ref_num_2 = &mut num;
}