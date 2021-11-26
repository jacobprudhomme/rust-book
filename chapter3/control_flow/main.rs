fn main() {
    // if Expressions
    let number = 3;

    // Despite if being an expression, it does not need an else branch
    if number != 4 {
        println!("number is not 4");
    }

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false...");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Condition must be a bool
    // if number {
    //     println!("number = {}", number);
    // }

    let result = if number > 1 { 5 } else { 6 };  // Since if is an expression, it can be assigned to a variable
    // let result = if number > 1 { 5 } else { "six" };  // Both branches must have same type


    // Loop Statements
    loop {  // Infinite loop
        println!("To infinity!");
        break;  // Break from loop (applies to innermost loop)
        continue;  // Skip rest of loop and go to next loop iteration (applies to innermost loop)
    }

    let mut count = 0;
    'counting_up: loop {  // Labeled loop
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;  // Breaks from this loop
            }
            if count == 2 {
                break 'counting_up;  // Breaks from outer, labeled loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut countdown = 3;
    while countdown != 0 {  // while loop
        println!("{}!", countdown);
        countdown -= 1;
    }
    println!("Blast-off!");

    let a = [10, 20, 30, 40, 50];
    for element in a {  // for loop
        println!("The value is: {}", element);
    }

    for time in (1..4).rev() {  // Using a range (in this case, reversed) for the loop variable
        println!("T-{} seconds!", time);
    }
    println!("Oh no... Houston, we have a problem.");

    // Loop Expressions
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Loop returns this value
        }
    };
    println!("The result is {}", result);


}
