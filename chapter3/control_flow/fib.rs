use std::io;

fn main() {
    loop {
        let mut num = String::new();

        println!("Which Fibonacci number do you want?");
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in numbers only.");
                continue;
            },
        };

        println!("The number is {}.", fib(num));

        break;
    }
}

fn fib(num: u64) -> u64 {
    let mut fib1 = 1;
    let mut fib2 = 1;

    for _ in 1..num {
        let temp = fib2;
        fib2 += fib1;
        fib1 = temp;
    }

    fib1
}
