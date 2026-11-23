// control flows
fn main() {
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // nested controls
    let number = 15;
    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is odd");
        if number > 10 {
            println!("{} is greater than 10", number);
        } else {
            println!("{} is not greater than 10", number);
        }
    }

    // use of && and ||
    let a: i32 = 10;
    let b: i32 = 5;
    let c: i32 = 20;

    if a > b && b > c {
        println!("a is greater than b and b is greater than c");
    } else {
        println!("The condition && is not met");
    }

    //controls with enum
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_of_coin(coin: Coin) -> i32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    println!("value_of_coin: {}", value_of_coin(quarter));

    // loops
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
