// Write a function that returns the first word in a string
// If the function finds no spaces in the string, the entire string must be a word, so the whole thing is returned
// What should we return? Don't really have a way to talk about PART of a string
// If we do the following and return the index of the end of the first word, have no way of guaranteeing that the returned value will be representative of the string we pass in in the future
// i.e. we can later on make the string empty, but the value we get from first_word() won't change in accordance, it desynchronizes. This can lead to errors!
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {  // iter() returns an iterator, enumerate() takes every element of the iterator and returns a tuple with its index and a reference to it
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    // Solution: string slices, a reference to just a portion of the string
    let s = String::from("Hello, world!");
    let hello = &s[0..5];  // Takes a slice of characters 0, 1, 2, 3, 4
    let world = &s[6..11];  // Takes a slice of characters 6, 7, 8, 9, 10
    // [0..x] can also be written [..x] and [y..<end>] can be written [y..]. [..] is a slice of the entire string

    // With our rewritten version of first_word(), the following will throw a compiler error, since clear() needs a mutable reference to s, but println!() uses the immutable reference returned from first_word2() afterwards.
    // The reference was created before clear(), so its scope surrounds that of the mutable reference
    let mut s = String::from("Hello, world!");
    let word = first_word2(&s);
    // s.clear();
    // println!("The first word is: {}", word);

    // Using our new, more general API using a &str parameter
    let s = String::from("Hello, world!");
    let word = first_word2(&s[0..6]);
    let word = first_word2(&s[..]);
    let word = first_word2(&s);  // A reference to a string is equivalent to a whole-string slice

    // Also works on slices of string literals
    let s_literal = "Hello, world!";
    let word = first_word2(&s_literal[0..6]);
    let word = first_word2(&s_literal[..]);
    let word = first_word2(s_literal);  // Since string literals are already slices of strings, passing it in just like this also works

    // We can also do array slices (and for other types of collections, as we'll see later)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // This has type &[i32]
    assert_eq!(slice, &[2, 3]);
}

// This version of first_word() returns a value tied to the underlying data in the string. The compiler will ensure references into the string remain valid
// fn first_word2(s: &String) -> &str {  // &str is the type for string slices. This explains why it's used for string literals: it's a slice pointing at a specific address in the binary
fn first_word2(s: &str) -> &str {  // We can rewrite the signature like this to make the function more general: you can pass in a slice or a reference to an entire string (this takes advantage of deref coercions)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
