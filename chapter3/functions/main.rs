fn main() {
    println!("Hello, world!");
    print_labeled_measurements(5, '\'');
}

// Define print_labeled_measurements() after main(). Functions can be defined anywhere
// snake_case is convention for function and variable names
fn print_labeled_measurements(value: i32, unit_label: char) {  // Must declare types of params
    println!("The measurement is: {}{}", value, unit_label);
}

// Rust is expression-based, so most things are expressions
fn statements_and_expressions() {
    // Statements (do not return values, so cannot assign to variable)
    let x = 6;  // So cannot do let x = (let y = 5); or let x = y = 5;
    fn inner_fn() -> i32 { 5 }  // Function definition (note: return type is given here)

    // Expressions (can assign to variable)
    inner_fn();  // Function calls
    println!("Hello, world!");  // Macro calls
    {  // Blocks (return value of final expression, or early returns using "return" keyword. Same as functions)
        let x = 3;
        x + 1  // A semicolon here will turn this into a statement, returning nothing
    };
}

// Should return i32, but returns nothing due to semicolon
// fn error_fn(x: i32) -> i32 {
//     x + 1;
// }
