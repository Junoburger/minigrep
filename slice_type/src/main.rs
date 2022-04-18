fn main() {
    struct Point {
        x: u32,
        y: u32,
    }
    // struct Color (i8, i8, i8);

    let point1 = Point { x: 10, y: 10 };
    // let color1 =    Color(10, 10, 10);

    println!("point1.x = {}, point1.y = {}", point1.x, point1.y);

    let teststr = String::from("Hello world");

    println!("The first word is: {}", find_first_word(&teststr));
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
