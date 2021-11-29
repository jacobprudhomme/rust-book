fn main() {
    {                     // s is not valid here since it's not yet declared
        let s = "Hello";  // s is valid from here until end of scope (s is a string literal, so stored in program code)
    }                     // Scope ends, s is no longer valid

    {
        let mut s = String::from("Hello");  // Heap-allocated string type
        s.push_str(", world!");             // Can be mutated
        println!("{}", s);                  // Can be printed
    }                                       // When s goes out of scope, we can return the memory to the heap (like RAII). Implicitly done by special function drop()

    {
        let x = 5;  // Bind 5 to x
        let y = x;  // Make a copy of value in x and bind it to y. So 2 separate copies of data (implements Copy trait)

        // Using method describe above, Rust would call drop() for both s1 and s2, resulting in double-free error
        // What really happens: s1 becomes invalid after being assigned to s2 (we say s1 has moved into s2)
        // Rust will never automatically create "deep" copies of data, so any automatic copying like this is inexpensive
        let s1 = String::from("Hello");  // String consists of ptr to data on heap, length, and capacity
        let s2 = s1;                     // String data itself is copied, not memory allocated on heap. So both variables point to same location
        // This causes a compilation error about borrowing: println!("{}, world!", s1);
    }

    {
        let s1 = String::from("Hello");
        let s2 = s1.clone();  // This "deeply" copies string data (i.e. copies data on the heap too)

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    // Note: a type cannot implement the Copy trait if it or any of its parts have implemented the Drop trait
    // As a rule, any scalar value or group of scalar values (e.g. tuple) implements the Copy trait, and anything requiring allocation cannot

    // Semantics for passing to a function are similar to assigning to a variable:
    {
        let s = String::from("Hello");  // s comes into scope
        takes_ownership(s);             // s moves into the function, so no longer usable after this point (would throw compiler error if we tried to use it)

        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x is copied into the function, so still usable after this point
    }                                   // x and s go out of scope. Since s's value was moved, no deallocation happens

    // Semantics for returning from a function:
    {
        let s1 = gives_ownership();         // Moves return value into s1

        let s2 = String::from("mine");      // s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved into callee, which moves return value into s3
    }                                       // s1 and s3 go out of scope and are dropped. s2 was moved, so nothing happens

    // What if we want to return ownership of a variable passed into a function, as well as a return a value produced by the function itself?
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
    // But wait! There's a better way, using references
}

fn takes_ownership(str: String) {  // str comes into scope
    println!("{}", str);
}                                  // str goes out of scope and drop() is called, freeing memory behind str

fn makes_copy(int: i32) {  // int comes into scope
    println!("{}", int);
}                          // int goes out of scope, but nothing special happens

fn gives_ownership() -> String {
    let str = String::from("yours");  // str is created here
    str                               // str is returned and moves out to caller
}

fn takes_and_gives_back(str: String) -> String {  // str comes into scope
    str                                           // str is returned and moves out to caller
}

fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}
