// Note: iterators are one of Rust's zero-cost abstractions. Using them imposes no additional
// runtime overhead (they get compiled to roughly the same code as writing the loop logic yourself)
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // iter: produces iterator over immutable references
    // into_iter: iterator that takes ownership and returns owned values
    // iter_mut: iterate over mutable references

    // methods that consume the iterator e.g. `sum`
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();

    assert_eq!(total, 6);

    // chaining iterator methods
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // closures that capture their environment
    #[derive(PartialEq, Debug)]
    struct Stuff {
        value: u32
    }

    fn stuff_with_value(stuffs: Vec<Stuff>, value: u32) -> Vec<Stuff> {
        stuffs.into_iter()
            .filter(|s| s.value == value)
            .collect()
    }

    let stuffs = vec![
        Stuff { value: 10 },
        Stuff { value: 5 },
        Stuff { value: 10 }
    ];

    let my_stuff = stuff_with_value(stuffs, 10);

    assert_eq!(
        my_stuff,
        vec![
            Stuff { value: 10 },
            Stuff { value: 10 }
        ]
    );
}