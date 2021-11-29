#[derive(Debug)]  // This attribute automatically derives the Debug trait, allowing us to use :? format specifier in printing
struct Rectangle {
    width: u32,
    height: u32,
}

// We can split up these methods over several impl blocks. There's no need to here, but there is a use for this
impl Rectangle {  // Everything within an implementation block is associated to the named type
    // Methods are just like functions, but called within the context of a struct (or enum or trait), and their first parameter is always self (the struct instance)
    // Their main benefit is code organization and discovery
    fn area(&self) -> u32 {  // &self is short for self: &Self, where Self is an alias for the type the impl block is for
        self.width * self.height
    }

    // We can have methods with the same name as fields, the language knows how to differentiate the two
    // Methods with the same name as a field are usually defined to be getters (just return the field value). Rust does not automatically implement these
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width) && (self.height >= other.height)
    }

    // Methods are a specific form of associated function (called as such because they're associate with the type the impl block represents) with self as the first parameter
    // Here is a general associated function, that does not need an instance of Rectangle to act on
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
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
    println!("The area of the rectangle is {} square pixels.", area(&rect));  // Using function
    println!("The area of the rectangle is {} square pixels.", rect.area());  // Using method
    // Rust has automatic referencing and dereferencing for methods, i.e. rect automatically becomes &rect/&mut rect/* rect,
    // depending on the type given to self in the method signature, so we don't have to write things like (&rect).area()

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);  // This is how we call associated functions
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
