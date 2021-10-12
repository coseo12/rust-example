fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three")
    }
        
    #[derive(Debug)]
    enum UseState {
        Alabama
    }

    enum Coin {
        Quarter(UseState),
    }

    let mut count = 0;
    let coin = Coin::Quarter(UseState::Alabama);
    match coin {
        Coin::Quarter(state) => println!("State Quarter from {:?}!", state),
        _ => count += 1,
    }

    // let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
