// crypto.rs

pub fn encrypt(plaintext: &str, key: &str) -> String {
    let m = key.len() as u32;
    let mut ciphertext = String::new();
    let mut key_pos = 0;

    for character in plaintext.chars() {
        if !character.is_whitespace() {
            let shifted = (character as u32 + key.chars().nth(key_pos % m as usize).unwrap() as u32) % 26;
            ciphertext.push((shifted as u8 + b'A') as char);
            key_pos += 1;
        } else {
            ciphertext.push(character);
        }
    }

    ciphertext
}

pub fn decrypt(ciphertext: &str, key: &str) -> String {
    let m = key.len() as u32;
    let mut plaintext = String::new();
    let mut key_pos = 0;

    for character in ciphertext.chars() {
        if !character.is_whitespace() {
            let shifted = (character as u32 + 26 - key.chars().nth(key_pos % m as usize).unwrap() as u32) % 26;
            plaintext.push((shifted as u8 + b'A') as char);
            key_pos += 1;
        } else {
            plaintext.push(character);
        }
    }

    plaintext
}
