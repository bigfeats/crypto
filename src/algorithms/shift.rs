use std::ops::Add; // For adding to a String

pub fn encrypt(message: &String, key: &String) -> String {
    println!("Encrypting with Shift, message: {}, key: {}", message, key);

    let key = match key.parse::<i32>() {
        Ok(k) => k,
        Err(_message) => return format!("Invalid key for shift cipher: {}", key)
    };

    shift_string(message, key)
}

pub fn decrypt(message: &String, key: &String) -> String {
    println!("Decrypting with Shift, message: {}, key: {}", message, key);

    let key = match key.parse::<i32>() {
        Ok(k) => k,
        Err(_message) => return format!("Invalid key for shift cipher: {}", key)
    };

    shift_string(message, -key)
}

fn shift_string(message: &String, offset: i32) -> String {
    let mut ciphertext = String::new();

    for letter in message.chars() {
        let shifted_letter = shift_letter(letter, offset);

        ciphertext = ciphertext.add(shifted_letter.to_string().as_str());
    }

    ciphertext
}

fn shift_letter(plain_letter: char, offset: i32) -> char {
    let base = 'a' as u8;
    let as_number = plain_letter as u8 - base;
    let offset_number = as_number as i32 + offset;

    // % is a remainder function, not a modulus.  They work differently for negative numbers.
    let mod_offset = (((offset_number % 26) + 26) % 26) as u8;
    let shifted_char = (base + mod_offset) as char;

    shifted_char
}
