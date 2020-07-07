#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let n = value_in_cents(Coin::Nickel);
    let q = value_in_cents(Coin::Quarter(UsState::Arkansas));

    println!("{}", n);
    println!("{}", q);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?} and {:?}", five, six, none);

    // non exhaustive match
    let some_u8 = 8_u8;
    match some_u8 {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // control flow with `if let`
    let some_u8 = Some(3_u8);
    match some_u8 {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3_u8) = some_u8 {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
