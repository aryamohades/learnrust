use std::thread;
use std::time::Duration;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    let x = vec![1, 2, 3];

    // using the `move` keyword to force closure to take ownership of variables it captures from its environment
    let equal_to_x = move |z| z == x;

    // does not compile
    // println!("can't use x here: {:?}", x);
}

// there are three `Fn` traits:
// FnOnce: consumes the variables it captures from its enclosing scope, known as the closure's environment.
//      Can only be called once because the closure can't take ownership of the same variables more than once.
// FnMut: can change the environment becaues it mutably borrows values
// Fn: borrows values from the environment immutably

// When you create a closure, Rust infers which trait to use based on how the closure uses the values from the
// environment. All closures implement FnOnce because they can all be called at least once. Closures that don't
// move the captured variables also implement FnMut, and closures that don't need mutable access to the captured
// variables also implement Fn.

// If you want to force the closure to take ownership of the values it uses in the environment, you can use the
// `move` keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread
// to move the data so it's owned by the new thread.
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}