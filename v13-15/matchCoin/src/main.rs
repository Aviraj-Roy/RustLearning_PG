#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
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
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value is {}", value_in_cents(coin));
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value is {}", value_in_cents(coin));
}
