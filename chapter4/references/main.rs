fn main() {
    let s = String::from("Hello");
    let len = calculate_length(&s);  // Passed reference here (&), does not transfer ownership
    println!("The length of '{}' is {}.", s, len);
    // Note: opposite of referencing (using &) is dereferencing (using *)

    // References are immutable by default, like variables: change(&s1);

    let mut s = String::from("Hello");  // Make variable mutable
    change(&mut s);                     // We are passing a mutable reference (we expect s2 to be modified somewhere in change())

    // We cannot have more than one mutable reference to a particular piece of data at a time! (Only one person can borrow at a time)
    // Prevents data races at compile time (1-two or more pointers access same data at same time, 2-at least one of these 2 pointers is being used to write to the data, and 3-there's no mechanism synchronizing data access)
    let r1 = &mut s;
    // Will result in compiler error: let r2 = &mut s2;
    // println!("{}, {}", r1, r2);

    // We can have multiple mutable references as long as they're not simultaneous
    {
        let r1 = &mut s;
    }  // r1 goes out of scope here, so we can make a new reference after this point
    let r2 = &mut s;

    // We can have multiple immutable references (no chance for a data race), but not a mutable one while we hold any immutable ones, as it may affect people's reading of immutable data
    let r1 = &s;      // No problem
    let r2 = &s;      // No problem
    // Houston, we have a problem! let r3 = &mut s;
    // println!("{}, {} and {}", r1, r2, r3);

    // A reference's scope lasts from when it is created to the last point it is used
    // The following code compiles because the last usage of the immutable references is before the mutable reference is introduced
    // Compiler determines the point a reference is no longer in use with Non-Lexical Lifetimes (NLL)
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);  // r1 and r2 will not be used after this point. They become out of scope
    let r3 = &mut s;                // Scope of r3 begins
    println!("{}", r3);

    // The compiler ensures that data will not go out of scope before before references to it do (no dangling references)
    // let reference_to_nothing = dangle();
    let moved_value = no_dangle();

    // In summary:
    // - Can have multiple immutable references OR a single mutable reference at a single time
    // - References must always be valid
}

fn calculate_length(str: &String) -> usize {  // Accepts reference here (&), does not take ownership (this is called borrowing)
    str.len()
}                                             // str goes out of scope here, but since it doesn't have ownership of value it refers to, nothing happens

// Cannot modify value that is borrowed, it will throw a compiler error
// fn change(str: &String) {
//     str.push_str(", world!");
// }

fn change(str: &mut String) {  // Accepts a mutable reference (we will be modifying str somewhere in the function)
    str.push_str(", world!");
}

// This throws a compiler error about the return type containing a borrowed value when there is no value for it to be borrowed from
// fn dangle() -> &String {            // Attempt to create a dangling reference
    // let s = String::from("Hello");
    // &s                              // Return a reference to s
// }                                   // s goes out of scope and is dropped, so its memory goes away

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}  // Ownership of s is moved out to caller and nothing is deallocated
