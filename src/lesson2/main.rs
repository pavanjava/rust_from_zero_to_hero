fn main() {
    let x:i32 = 5; // by default all variables are immutable in rust if we declare a variable using let
    // to make them mutable add the key word mut, like let mut x = 5
    println!("The value of x is {}", x);

    // x = 6; // raises error: 'cannot assign twice to immutable variable'
    // println!("The value of x is {}", x);

    let mut y = 10; // once a mutable variables are assigned then the variable is tightly
    // coupled with ots datatype and can never be changed down stream
    println!("The value of y is {}", y);

    // y = "Hello to new variable"; // raises error: expected integer, found `&str`
    // println!("The value of y is {}", y);

    let z = 30;
    println!("The value of z is {}", z);

    { // shadow the variable in a inner scope
        let z = 10;
        println!("The value of z is {}", z);
    }

    let z = z + 20; // can use the same variable in same scope multiple times
    println!("The value of z is {}", z);

    // constants
    // 1. Const can not be mutable
    // 2. Const can not be shadowable
    // 3. const can not be declared without a data type
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

}