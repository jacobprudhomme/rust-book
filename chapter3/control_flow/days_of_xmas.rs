fn main() {
    for day in 1..=12 {
        print_first_line(day);
        print_rest(day);
        if day != 12 {
            println!();
        }
    }
}

fn print_first_line(day: u8) {
    let which_day = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!("On the {} day of Christmas, my true love gave to me", which_day);
}

fn print_rest(day: u8) {
    if day == 12 {
        println!("Twelve drummers drumming,");
    }
    if day >= 11 {
        println!("Eleven pipers piping,");
    }
    if day >= 10 {
        println!("Ten lords a-leaping,");
    }
    if day >= 9 {
        println!("Nine ladies dancing,");
    }
    if day >= 8 {
        println!("Eight maids a-milking,");
    }
    if day >= 7 {
        println!("Seven swans a-swimming,");
    }
    if day >= 6 {
        println!("Six geese a-laying,");
    }
    if day >= 5 {
        println!("Five goooldeeen rings,");
    }
    if day >= 4 {
        println!("Four calling birds,");
    }
    if day >= 3 {
        println!("Three French hens,");
    }
    if day >= 2 {
        println!("Two turtle doves,");
    }
    if day == 1 {
        println!("A partridge in a pear tree.");
    } else {
        println!("And a partridge in a pear tree.")
    }
}
