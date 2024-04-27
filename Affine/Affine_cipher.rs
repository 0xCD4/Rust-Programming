fn main() {
    let plaintext = "Hacker";
    let key_a = 5;
    let key_b = 8;

    let ciphertext = affine_encrypt(plaintext, key_a, key_b);
    println!("Ciphertext: {}", ciphertext);

    let decrypted_text = affine_decrypt(&ciphertext, key_a, key_b);
    println!("Decrypted text: {}", decrypted_text);
}

fn affine_encrypt(plaintext: &str, key_a: i32, key_b: i32) -> String {
    let mut ciphertext = String::new();

    for ch in plaintext.chars() {
        if ch.is_alphabetic() {
            let ch = ch.to_ascii_uppercase();
            let ch_index = ch as i32 - 'A' as i32;
            let encrypted_index = (key_a * ch_index + key_b) % 26;
            let encrypted_ch = (encrypted_index + 'A' as i32) as u8 as char;
            ciphertext.push(encrypted_ch);
        }
    }

    ciphertext
}

fn affine_decrypt(ciphertext: &str, key_a: i32, key_b: i32) -> String {
    let mut decrypted_text = String::new();
    let key_a_inverse = mod_inverse(key_a, 26).unwrap();

    for ch in ciphertext.chars() {
        if ch.is_alphabetic() {
            let ch = ch.to_ascii_uppercase();
            let ch_index = ch as i32 - 'A' as i32;
            let decrypted_index = (key_a_inverse * (ch_index - key_b + 26)) % 26;
            let decrypted_ch = (decrypted_index + 'A' as i32) as u8 as char;
            decrypted_text.push(decrypted_ch);
        }
    }

    decrypted_text
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 1;
    let mut last_x = 1;
    let mut last_y = 0;
    let mut aa = a;
    let mut mm = m;

    while mm != 0 {
        let quotient = aa / mm;
        let remainder = aa % mm;

        aa = mm;
        mm = remainder;

        let temp = x;
        x = last_x - quotient * x;
        last_x = temp;

        let temp = y;
        y = last_y - quotient * y;
        last_y = temp;
    }

    if aa > 1 {
        None // Modular inverse does not exist
    } else {
        Some((last_x + m) % m)
    }
}
