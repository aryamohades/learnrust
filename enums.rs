#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    // basic enum
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    // enum with field types
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    // enum with constant values
    enum DirectionWithValue {
        Left = 1,
        Right = 2,
        Up = 3,
        Down = 4,
    }

    // get enum value
    assert_eq!(DirectionWithValue::Left as u8, 1);

    // function that takes a `WebEvent` enum as an argument and
    // returns a string representation of the event with details
    fn debug(event: WebEvent) -> String {
        match event {
            WebEvent::PageLoad => "PageLoad".to_owned(),
            WebEvent::PageUnload => "PageUnload".to_owned(),
            WebEvent::KeyPress(c) => format!("KeyPress: {}", c),
            WebEvent::Paste(s) => format!("Paste: {}", s),
            WebEvent::Click { x, y } => format!("Click: {{ x: {}, y: {} }}", x, y)
        }
    }

    // awesome enum
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click { x: i64, y: i64 }
    }

    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("hello world".to_owned());
    let click = WebEvent::Click { x: 50, y: 100 };

    assert_eq!(debug(load), "PageLoad");
    assert_eq!(debug(unload), "PageUnload");
    assert_eq!(debug(pressed), "KeyPress: x");
    assert_eq!(debug(paste), "Paste: hello world");
    assert_eq!(debug(click), "Click: { x: 50, y: 100 }");

    enum DogKind {
        Doggo,
        Doggy,
        Doge,
        Pupper
    }

    // do the same thing for different enum values using the `|` operator
    let result = match DogKind::Doggy {
        DogKind::Doggo | DogKind::Doggy => "good doge",
        _ => "bad doge"
    };

    assert_eq!(result, "good doge");
}