const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  // Constants (as opposed to immutable variables) can be declared in any scope, must be type-annotated, and must be set to constant expression at compile-time (not init'able at runtime)

fn main() {
    // Reassign to mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadow existing variable
    let y = 5;  // x = 5
    let y = y + 1;  // x = 6. Uses old value of x in definition
    { // Starts new block (new nested scope)
        let y = y * 2;  // x = 12. Shadows x only in this scope
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // Shadowing can change type
    let spaces = "   ";
    let spaces = spaces.len();

    // Cannot reassign variable as different type
    // let mut spaces_mut = "   ";
    // spaces = spaces_mut.len();
}
