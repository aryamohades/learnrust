// Specify a function as a test using the #[test] attribute
// Ignore a particular test during `cargo test` using the #[ignore] attribute

// Run test and println! to terminal
// $ cargo test -- --nocapture

// Run test with a certain number of threads
// $ cargo test -- --test-threads=1

// Run one test
// $ cargo test <name of test fn>
// $ cargo test it_works

// Filter tests to run
// $ cargo test add
// this will run all tests that start with 'add' e.g. 'add_two_and_two' and 'add_three_and_two' would both run

// Run ignored tests
// $ cargo test -- --ignored

// The convention is to create a module named tests in each file to contain the test functions
// and to annotate the module with #[cfg(test)]

// #[cfg(test)] annotation tells Rust to compile the tests only when running `cargo test`
// `cargo build` will not include the tests to compile faster and save space

// To create integration tests, you first need a tests directory at the top level of your project next to src.
// We can then make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

// Run a particular integration test
// $ cargo test --test integration_test

// Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.
// e.g. tests/common/mod.rs
// Now we can put some setup code or any common task in the mod.rs and use it in different integration tests and common/mod.rs
// itself won't be compiled as a separate crate or have sections in the test output.

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
