// DATA TYPES
// Scalar Types
// - Integers
// - Floating-point numbers
// - Booleans
// - Characters

// Compound Types
// - Tuples
// - Arrays

// Custom Types
// - Structs
// - Enums

struct Person {
    name: String,
    age: u8,
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    // INTEGERS
    // Length   Signed  Unsigned
    // 8-bit    i8      u8
    // 16-bit   i16     u16
    // 32-bit   i32     u32
    // 64-bit   i64     u64
    // 128-bit  i128    u128
    // arch     isize   usize
    let _small_number: u8 = 255;
    let _big_number: u128 = 123456789012345678;
    let _small_number: i8 = -127;
    let _big_number: i128 = -123456789012345678;

    println!("small_number: {}", _small_number);
    println!("big_number: {}", _big_number);
    println!("small_number: {}", _small_number);
    println!("big_number: {}", _big_number);

    // Numeral System   Description Example
    // Decimal  Base-10, common form    98_222
    // Hexadecimal  Base-16, prefixed with 0x   0xff
    // Octal    Base-8, prefixed with 0o    0o77
    // Binary   Base-2, prefixed with 0b    0b1111_0000
    // Byte (u8 only)   ASCII characters, prefixed with b   b'A'

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("decimal: {}", decimal);
    println!("hex: {}", hex);
    println!("octal: {}", octal);
    println!("binary: {}", binary);
    println!("byte: {}", byte);

    // FLOATING POINTS
    let x = 2.0; // f64 default
    let y: f32 = 3.0; // f32

    println!("x = {}, y = {}", x, y);

    // NUMERIC OPERATIONS
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y; // division
    let remainder = 43 % 5; // remainder

    println!("sum = {}", sum);
    println!("difference = {}", difference);
    println!("product = {}", product);
    println!("quotient = {}", quotient);
    println!("remainder = {}", remainder);

    // BOOLEAN TYPE

    let t = true; // implicit declaration
    let f: bool = false; // explicit declaration

    println!("t = {}, f = {}", t, f);

    // if
    if t {
        println!("t is true");
    } else {
        println!("t is false");
    }

    let not_true = !t;
    println!("not_true = {}", not_true);

    // Characters
    let c = '𝕏';
    println!("c = {}", c);

    for char in "नमस्ते 🙏".chars() {
        println!("{}", char);
    }

    // Tuples
    let tup: (i32, f64, char) = (500, 6.4, 'x');

    // Accessing by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let x_char = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of x_char is: {}", x_char);

    // ARRAYS
    let arr = [1, 2, 3, 4, 5]; // array of 5 elements

    let first = arr[0]; // first element
    let second = arr[1]; // second element

    println!("first: {}, second: {}", first, second);

    for element in arr.iter() {
        println!("element: {}", element);
    }

    // STRUCTS
    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("name is {}", person.name);
    println!("age is {}", person.age);

    // Enums
    let light = TrafficLight::Yellow;

    match light {
        TrafficLight::Yellow => println!("Light Yellow"),
        TrafficLight::Green => println!("Light Green"),
        TrafficLight::Red => println!("Light Red"),
    }
}