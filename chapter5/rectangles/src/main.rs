#[derive(Debug)]  // This attribute automatically derives the Debug trait, allowing us to use :? format specifier in printing
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // Attempt 1:
    // let width = 30;
    // let height = 50;

    // Attempt 2:
    // let rect = (30, 50);

    // Attempt 3:
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),  // dbg!() is a macro that takes ownership of the inner expression, prints the file, line number, and expression to stderr, then returns ownership (so it's like just putting the value there)
        height: 50,
    };

    // We get a compiler error that Rectangle doesn't implement the trait Display ({} uses this formatting): println!("rect is {}", rect);
    println!("rect is {:?}", rect);  // We can use the :#? format specifier when structs get too big (it breaks up the output with each field to its own line)
    dbg!(&rect);  // We don't want to pass ownership of the rectangle to dbg!(), so we use a reference (we could also reassign the result back to rect, if it were immutable)


    // Attempt 1:
    // println!("The area of the rectangle is {} square pixels.", area(width, height));

    // Attempt 2:
    // println!("The area of the rectangle is {} square pixels.", area(rect));

    // Attempt 3:
    println!("The area of the rectangle is {} square pixels.", area(&rect));
}

// Attempt 1:
// fn area(width: u32, height: u32) -> u32 {

// Attempt 2:
// fn area(dimensions: (u32, u32)) -> u32 {

// Attempt 3:
fn area(rectangle: &Rectangle) -> u32 {
    // Attempt 1:
    // width * height

    // Attempt 2:
    // dimensions.0 * dimensions.1

    // Attempt 3:
    rectangle.width * rectangle.height
}
