fn main() {
    let fave_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fave_color {
        println!("Using your favorite color, {color}, as the background");
    }
    else if is_tuesday {
        println!("Tuesday is green day!");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }
        else {
            println!("Using orange as the background color");
        }
    }
    else {
        println!("Using blue as the background color");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({x}, {y})");
    }

    let point = (3, 5);
    print_coordinates(&point);

    let x = 1;
    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        4..=10 => println!("4 through 10"),
        _ => println!("anything else")
    }

    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let point = Point {x: 0, y: 7, z: 0};
    let Point {x, y, z} = point;
    assert_eq!(0, x);
    assert_eq!(7, y);
    assert_eq!(0, z);

    match point {
        Point {x, y: 0, z: _} => println!("On the x axis at {x}"),
        Point {x: 0, y, z: _} => println!("On the y axis at {y}"),
        Point {x, y, z: _} => println!("On neither axis: ({x}, {y})")
    }


    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32)
    }

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Message: Change color to (r, g, b) = ({r}, {g}, {b})"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Message: Change color to (h, s, v) = ({h}, {s}, {v})"),
        _ => (),
    }

    let origin = Point {x: 0, y: 0, z: 0};
    match origin {
        Point{x, ..} => println!("x: {x}")
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => println!("First and last of sequence: {first}, {last}")
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("{x} exists in `num` and is even"),
        Some(x) => println!("{x} exists in `num` and is odd"),
        None => println!("No number exists in `num`")
    }

    enum OtherMessage {
        Hello { id: i32 }
    }

    let msg = OtherMessage::Hello { id: 5 };

    match msg {
        OtherMessage::Hello {
            id: id_variable @ 3..=7
        } => println!("Found an id in range 3..=7: {id_variable}"),
        OtherMessage::Hello { id: 10..=12 } => println!("Found an  id in another range"),
        OtherMessage::Hello { id } => println!("Found some other id: {id}")
    }
}
