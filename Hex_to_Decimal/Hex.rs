use std::io::{self, Write};

fn decimal_to_hex(mut num: u32) -> String {
    if num == 0 {
        return "0".to_string();
    }

    const HEX_CHARS: &str = "0123456789ABCDEF";
    let mut hex_string = String::new();

    while num > 0 {
        let remainder = num % 16;
        hex_string.push(HEX_CHARS.chars().nth(remainder as usize).unwrap());
        num /= 16;
    }

    hex_string.chars().rev().collect()
}

fn main() {
    print!("Enter a decimal number: ");
    io::stdout().flush().unwrap();  // Ensure the prompt is displayed before reading input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let decimal_number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input, please enter a valid decimal number.");
            return;
        }
    };

    let hex_string = decimal_to_hex(decimal_number);
    println!("The hexadecimal representation of {} is {}", decimal_number, hex_string);
}
