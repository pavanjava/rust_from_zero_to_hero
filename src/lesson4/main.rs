fn integers_in_rust(){
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
}

fn numerical_system(){
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
}

fn compute(num: i32) -> i32 {
    return num * 2
}

fn main() {
    integers_in_rust();
    numerical_system();
    println!("The computed response = {}", compute(10));

    let y = {
        let x = 5;
        x + 1 // without semicolon its expression but with semicolon it becomes statement
    };
    println!("y = {}", y);
}