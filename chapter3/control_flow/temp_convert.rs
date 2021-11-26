use std::io;

fn main() {
    loop {
        let mut unit = String::new();

        println!("Enter your source units (C or F):");
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        let unit: char = match unit.trim().parse() {
            Ok(char) => if char == 'C' || char == 'F' {
                char
            } else {
                continue;
            },
            Err(_) => continue,
        };

        let mut temp = String::new();

        println!("Enter your temperature:");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if unit == 'C' {
            println!("{}°C -> {}°F", temp, c_to_f(temp));
        } else {
            println!("{}°F -> {}°C", temp, f_to_c(temp));
        }

        break;
    }
}

fn c_to_f(temp: f32) -> f32 {
    (temp * 9.0 / 5.0) + 32.0
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}
