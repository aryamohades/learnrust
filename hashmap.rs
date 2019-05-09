use std::collections::{HashMap, hash_map::Entry::{Occupied, Vacant}};

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // basic hashmap
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("foo".to_string(), 42);

    // check for existence of key
    assert_eq!(map.contains_key("foo"), true);
    assert_eq!(map.contains_key("bar"), false);

    // get value of key, returns Option
    assert_eq!(map.get("foo"), Some(&42));
    assert_eq!(map.get("bar"), None);

    // remove key
    map.remove("foo");
    assert_eq!(map.get("foo"), None);

    // insert new value or append to old value
    fn insert_or_append(map: &mut HashMap<String, String>, s: String) {
         match map.entry("foo".to_string()) {
            Vacant(entry) => { entry.insert(s); },
            Occupied(mut entry) => { entry.get_mut().push_str(&s); }
        }
    }

    // insert new value or increment existing value
    fn insert_or_increment(map: &mut HashMap<&str, i32>, num: i32) {
        match map.entry("foo") {
            Vacant(entry) => { entry.insert(num); },
            Occupied(mut entry) => { *entry.get_mut() += 1; }
        }
    }

    let mut map = HashMap::<String, String>::new();
    insert_or_append(&mut map, "foo".to_string());
    assert_eq!(map.get(&"foo".to_string()), Some(&"foo".to_string()));
    insert_or_append(&mut map, "bar".to_string());
    assert_eq!(map.get(&"foo".to_string()), Some(&"foobar".to_string()));    

    let mut map: HashMap<&str, i32> = HashMap::new();
    insert_or_increment(&mut map, 42);
    assert_eq!(map.get("foo"), Some(&42));
    insert_or_increment(&mut map, 42);
    assert_eq!(map.get("foo"), Some(&43));

    // iterate over keys and values
    for (k, v) in &map {
        // do something with `k` and `v`
    }

    // the type can be inferred, `HashMap<&str, u8>` in this case
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u8 {
        42
    }

    // insert a key only if it doesn't already exist
    player_stats.entry("health").or_insert(100);
    player_stats.entry("health").or_insert(200);
    assert_eq!(player_stats.get("health"), Some(&100));

    // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("defense").or_insert_with(random_stat_buff);
    player_stats.entry("defense").or_insert(50);
    assert_eq!(player_stats.get("defense"), Some(&42));

    // update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();

    #[derive(Hash, Eq, PartialEq, Debug)]
    struct Viking {
        name: String,
        country: String,
    }

    impl Viking {
        // Viking constructor
        fn new(name: &str, country: &str) -> Viking {
            Viking { name: name.to_string(), country: country.to_string() }
        }
    }

    // hashmap to store the vikings' health points.
    let mut vikings = HashMap::new();

    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    for (viking, health) in &vikings {
        // do something with `viking` and `health`
    }

    // build a hashmap from an array of tuples
    let timber_resources: HashMap<&str, i32> = [
        ("Norway", 100),
        ("Denmark", 50),
        ("Iceland", 10)
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(timber_resources.get("Norway"), Some(&100));

    let teams = vec!["red".to_owned(), "blue".to_owned(), "green".to_owned()];
    let scores = vec![50, 100, 150];

    // create map from teams and scores while moving teams and scores in the process
    // if we wanted references and wanted to continue using teams and scores,
    // we could use .iter() instead of .to_iter()
    let map: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();
    assert_eq!(map.get("blue"), Some(&100));

    // does not compile
    // println!("{:?}", teams);
}