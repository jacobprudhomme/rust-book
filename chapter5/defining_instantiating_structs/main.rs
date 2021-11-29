struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Can also define tuple structs, which are like structs with nameless fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Can create structs isomorphic to unit (() type), with no fields. Can be useful for implementing a trait on some type that doesn't actually store data
// All instances of this struct will always be equal
struct AlwaysEqual;

fn main() {
    // Entries can be listed in a different order than defined
    // We use the owned String type instead of string slices (&str) because we want an instance of a struct to own all of its data, so its valid for as long as the struct is valid
    // Not doing so results in a compiler error that needs to be solved with lifetimes (a future topic)
    let user1 = User {
        email: String::from("jacobprudhomme@users.noreply.github.com"),
        username: String::from("jacobprudhomme"),
        active: true,
        sign_in_count: 1,
    };
    println!("My username: {}", user1.username);  // Access fields using dot-notation

    // Note: using the mut keyword, the entire instance becomes mutable. We cannot pick and choose mutable fields
    let mut user1 = User {
        email: String::from("jacobprudhomme@users.noreply.github.com"),
        username: String::from("jacobprudhomme"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@email.com");  // Mutate fields using dot-notation

    // Struct update syntax (says that all field not explicitly set should take their value from the instance of the struct after the "..")
    let user2 = User {
        email: String::from("another_other@email.com"),
        ..user1  // Must come last!
    };
    // Note: uses move semantics, like with variable assignment. So the username field of user1 becomes invalid!

    // These tuple structs have entirely different types despite being so similar, since each struct forms a type
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Tuple structs can be destructured and have indexed access, just like regular tuples

    let subject = AlwaysEqual;
}

// We can return instances of structs
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username,  // We can use field init shorthand syntax when the field and variable have the same name
        active: true,
        sign_in_count: 1,
    }
}
