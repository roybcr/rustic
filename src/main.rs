use crate::animal::{Human, Pet, PetOwner};

mod animal;

fn main() {
    let dog: Pet = Pet::new("Dog".to_owned(), "Wuff!".to_owned());
    let me: Human = Human::new("Roy".to_owned(), Some(dog));
    me.go_for_a_walk();

    // Instantiate a `Point`
    let _ne: Point = Point { x: 1.0, y: 3.0 };
    let _ne_: Point = Point { .._ne };

    let _rectangle: Rectangle = Rectangle {
        ne: Point::from(_ne),
        sw: Point {
            x: match _ne_.x {
                xf32 if xf32 > 0.0 => xf32 * -1.0,
                xf32 => xf32,
            },
            y: match _ne_.y {
                y if y > 0.0 => y * -1.0,
                y => y,
            },
        },
    };

    let area: f32 = rect_area(Rectangle::from(_rectangle));

    println!(
        "Calculated area for the rectangle {:#?} is {:?}",
        _rectangle, area
    );

    let load: WebEvent = WebEvent::PageLoad;
    let unload: WebEvent = WebEvent::PageUnload;
    let pressed: WebEvent = WebEvent::KeyPress('x');
    let pasted: WebEvent = WebEvent::Paste("my text".to_owned());
    let click: WebEvent = WebEvent::Click { x: 30, y: 50 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(click);
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded!"),
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\". ", s),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}.", x, y),
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]

struct Rectangle {
    ne: Point,
    sw: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    // Nested destructuring !
    let Rectangle {
        ne: Point { x: x_ne, y: y_ne },
        sw: Point { x: x_sw, y: y_sw },
    } = rect;

    let base_length: f32 = y_ne - y_sw;
    let diagonal_length: f32 = x_ne - x_sw;

    return base_length * diagonal_length / 2.0;
}
