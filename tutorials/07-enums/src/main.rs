
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Wisconsin,
    Wyoming,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter: {state:?}");
            25
        },
    }
}

fn add_fancy_hat() {
    println!("You got a fancy hat!");
}

fn remove_fancy_hat() {
    println!("You lost your fancy hat!");
}

fn reroll() {
    println!("You rerolled the dice.");
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    // From this point, we know that state is valid
    println!("State quarter: {state:?}");
    Some(format!("{state:?}"))
}

fn main() {
    println!("Value of a dime: {} cents", value_in_cents(Coin::Dime));
    println!("Value of a quarter from Alaska: {} cents", value_in_cents(Coin::Quarter(UsState::Alaska)));
    describe_state_quarter(Coin::Quarter(UsState::Alabama));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let config_max = Some(3u8);
    // Equivalent to: match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } // could add an else if we want
}
