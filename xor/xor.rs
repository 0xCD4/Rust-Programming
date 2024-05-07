use std::env;

fn xor(plaintext: &str, key: char) -> String {
    plaintext.chars().map(|c| (c as u8 ^ key as u8) as char).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <plaintext> <key>", args[0]);
    } else {
        let plaintext = &args[1];
        let key = args[2].chars().next().unwrap();
        let ciphertext = xor(plaintext, key);
        println!("Ciphertext: {}", ciphertext);
    }
}
