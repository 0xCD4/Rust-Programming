fn rot13(input: &str) -> String {
    input.chars()
        .map(|c| {
            match c {
                'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
                'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
                _ => c,
            }
        })
        .collect()
}

fn main() {
    let plaintext = "Hello, Hacker";
    let ciphertext = rot13(plaintext);
    println!("Ciphertext: {}", ciphertext);
}
