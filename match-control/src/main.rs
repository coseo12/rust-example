#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let p = value_in_cents(Coin::Penny);
    let n = value_in_cents(Coin::Nickel);
    let d = value_in_cents(Coin::Dime);
    let q1 = value_in_cents(Coin::Quarter(UsState::Alabama));
    let q2 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{} {} {} {} {}", p, n, d, q1, q2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?} {:?} {:?}", five, six, none);

    let some_u8_value = 0u8;
    let m = match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    };

    println!("{:?}", m);
}
