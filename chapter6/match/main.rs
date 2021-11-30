enum Coin {
    Nickel,
    Dime,
    Quarter,
    Loonie(bool),
    Toonie,
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn do_something() {}
    fn do_something_else() {}
    fn do_something_with_other(other: u8) {}
    fn do_nothing() {}
    let dice_roll = 9;
    match dice_roll {
        3 => do_something(),
        7 => do_something_else(),
        other => do_something_with_other(other),  // This last pattern catches every other possible value and binds it to the variable name other. It must always go last, since patterns are evaluated in order
        // If we don't care about capturing the catch-all value, we can use a wildcard: _ => do_nothing(),
        // If we don't want to execute any code for a branch, we can use the unit value: _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    // match allows comparison of a value against a series of patterns, which can be literal values, a variable, wildcards, and many others
    // The compiler can confirm if all possible cases are handled. The first pattern the value matches against is the arm that is selected
    match coin {
        Coin::Nickel => 5,  // => separates the pattern (on the left) and the code to run (on the right)
        Coin::Dime => {  // {} can also be used to surround the code block (and must be, for multi-line blocks)
            println!("Also known as the Bluenose.");
            10
        },
        Coin::Quarter => 25,
        Coin::Loonie(sesquicentennial) => {  // We can bind variables to the data contained within variants too
            if sesquicentennial {
                println!("Canada's 150th anniversary loonie!");
            }
            100
        },
        Coin::Toonie => 200,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match works the exact same way for Option<T>, because it's just an enum
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// This won't compile because we didn't handle the None case (matches in Rust are exhaustive)
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }
