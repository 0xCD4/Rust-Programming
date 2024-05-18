fn main() {
    let key = b"secret";
    let plaintext = b"Hacker";
    let ciphertext = rc4(key, plaintext);
    let decrypted = rc4(key, &ciphertext);

    println!("Ciphertext: {:?}", ciphertext);
    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
}
fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s: Vec<u8> = (0..=255).collect();
    let key_len = key.len();

    let mut j = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key_len] as usize) % 256;
        s.swap(i, j);
    }

    
    let mut i = 0;
    let mut result = Vec::with_capacity(data.len());

    for &byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k = s[(s[i] as usize + s[j] as usize) % 256];
        result.push(byte ^ k);
    }

    result
}
