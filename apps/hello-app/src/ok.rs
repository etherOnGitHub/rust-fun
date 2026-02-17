#[derive(Debug)]
struct Struct<T>(T);
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

pub fn slicing() {
    println!("Slicing function called");
    let vec = vec![1, 2, 3, 4, 5];
    let int = &vec[..];
    let string: &[&str] = &["Hello", "World"];

    println!("The vec slice is: {:?}", vec);
    println!("The int slice is: {:?}", int);
    println!("The string slice is: {:?}", string);
    assert_eq!(string.len(), 2);
    assert!(!string.is_empty());

    let string = "Hello, Rust!";
    let string2 = "Nice
     to 
     meet 
     you!";
    let concat = format!("{} {}", string, string2);
    println!("The concatenated string is: {}", concat);

    // concat multi string + with string literal
    let stringy1 = "Hello, 
    Program is running! ";
    let stringy2 = "Rust!";
    let concat_string = stringy1.to_string() + stringy2;
    println!("The concatenated string is: {}", concat_string);

    let mut stringy3 = "bruh".to_string();
    stringy3.push_str(" moment");
    println!("The string after push_str is: {}", stringy3);

    let tuple = ("one", "two", "three");
    println!("The tuple is: {:?}", tuple);
    println!("The first element of the tuple is: {}", tuple.0);
    println!("The second element of the tuple is: {}", tuple.1);
    println!("The third element of the tuple is: {}", tuple.2);

    let structure: Struct<char> = Struct('A');
    println!("The structure is: {:?}", structure);
    let struct2: Struct<i32> = Struct(42);
    println!("The structure with i32 is: {:?}", struct2);

    let point: Point = Point { x: 6.0, y: 7.0 };
    println!("The point is: {} {}", point.x, point.y);
}

struct Coordinates {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Coordinates,
    p2: Coordinates,
}

struct TupleStruct(i32, f64);

pub fn rectalarea() {
    let point1 = Coordinates { x: 3.0, y: 6.0 };
    let point2 = Coordinates { x: 12.0, y: 17.0 };

    println!("The coordinates of point1 are: {} {}", point1.x, point1.y);
    println!("The coordinates of point2 are: {} {}", point2.x, point2.y);

    let _rectangle = Rectangle { p1: point1, p2: point2 };

    println!("p1: {}, {}, p2: {}, {}", _rectangle.p1.x, _rectangle.p1.y, _rectangle.p2.x, _rectangle.p2.y);

    let height = _rectangle.p2.y - _rectangle.p1.y;
    let width = _rectangle.p2.x - _rectangle.p1.x;
    let area = height * width;
    println!("The area of the rectangle is: {}", area);

    let pair = TupleStruct(42, 3.14);
    println!("The tuple struct is: ({}, {})", pair.0, pair.1);
    let TupleStruct(int, dec) = pair;
    println!("The int is: {:?}", int);
    println!("The dec is: {:?}", dec);
}

pub fn main() {
    slicing();
    rectalarea();
}