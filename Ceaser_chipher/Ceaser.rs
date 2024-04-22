fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        let encrypted_char = match ch {
            'A'..='Z' => ((ch as u8 - b'A' + key) % 26 + b'A') as char,
            'a'..='z' => ((ch as u8 - b'a' + key) % 26 + b'a') as char,
            _ => ch, 
        };
        result.push(encrypted_char);
    }

    result
}

fn main() {
    let plaintext = "Hello, Hacker";
    let key = 5;
    println!("Plaintext: {}", plaintext);
    let ciphertext = encrypt(plaintext, key);
    println!("Ciphertext: {}", ciphertext);
    let decrypted_text = encrypt(&ciphertext, 26 - key); // Decryption key = 26 - encryption key
    println!("Decrypted: {}", decrypted_text);
}
