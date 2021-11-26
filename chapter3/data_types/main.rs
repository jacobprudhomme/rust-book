use std::io;

fn main() {
    // Scalar types (single value)

    // -- Integers
    // ---- Signed ints (number is bit-width of int)
    let x: i8 = -1;
    let x: i16 = -2;
    let x: i32 = -3;
    let x: i64 = -4;
    let x: i128 = -5;
    let x: isize = -6;  // Bit-width of current architecture program is running on
    // ---- Unsigned ints (number is bit-width of int)
    let x: u8 = 1;
    let x: u16 = 2;
    let x: u32 = 3;
    let x: u64 = 4;
    let x: u128 = 5;
    let x: usize = 6;
    // ---- Integer literats
    let x = 127u8;  // Type suffix (u8 in this example)
    let x = 1_000_000;  // Visual separator (_)
    let x = 1234;  // Decimal (default)
    let x = 0xff;  // Hex (0x prefix)
    let x = 0o77;  // Octal (0o prefix)
    let x = 0b1111_0000;  // Binary (0b prefix)
    let x = b'A';  // Byte (can only be u8)
    // Note: integer over/underflow causes panic in debug mode. There exist methods to explicitly handle this (wrapping_*, checked_*, overflowing_*, saturating_*)

    // -- Floating-point numbers
    let x: f32 = 1.0;  // Single-precision float
    let x: f64 = 2.0;  // Double

    // Numeric operations
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;  // Floating-point division
    let div = 2 / 3;  // Integer division (rounds down)
    let rem = 43 % 5;
    // Cannot use operators on 2 different types: let sum = 5 + 10.0;

    // -- Booleans (use 1 byte)
    let val = true;
    let val: bool = false;

    // -- Characters (char literals use single-quotes, use 4 bytes, supports Unicode)
    let c = 'z';
    let c = 'ℤ';
    let c: char = '👀';


    // Compound types (multiple values grouped into one type)

    // -- Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // Destructuring
    let first_item = tup.0;  // Access by index
    let second_item = tup.1;
    let third_item = tup.2;
    let unit: () = ();  // Unit value with unit type. Implicitly returned from expressions that don't return any other values

    // -- Arrays (fixed-length)
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];  // Array with 5 items, all initialized to 3
    let first_item = a[0];
    let second_item = a[1];
    // Causes out-of-bounds compiler error: let oob = a[7];
    // Following will cause runtime panic if index > 4 is entered
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let oob = a[index];
}
