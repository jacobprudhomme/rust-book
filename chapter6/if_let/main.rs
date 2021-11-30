enum Coin {
    Nickel,
    Dime,
    Quarter,
    Loonie(bool),
    Toonie,
}

fn main() {
    // If you want to match one pattern but ignore the rest, if let syntax is less verbose than match
    let config_max = Some(3u8);  // Type suffix
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // vs.
    if let Some(max) = config_max {
        println!("the maximum is configured to be {}", max);
    }
    // Note: we lose the exhaustive checking of match!

    let mut count = 0;
    let coin = Coin::Quarter;
    if let Coin::Loonie(_) = coin {
        println!("You've got a dollar.");
    } else {  // Like the _ wildcard case of a match statement
        count += 1;
    }
}
