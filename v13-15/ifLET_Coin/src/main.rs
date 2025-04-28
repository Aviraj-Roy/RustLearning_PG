#[derive(Debug)] // so we can inspect the state
enum UsState {
    Alabama,
    Alaska,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)] // Needed for printing Coin::Quarter(state)
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
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

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("Value is {}", value_in_cents(&coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value is {}", value_in_cents(&coin));

    if let Some(description) = describe_state_quarter(&coin) {
        println!("{description}");
    }
}
