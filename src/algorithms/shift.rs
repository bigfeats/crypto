use std::ops::Add; // For adding to a String
use algorithms::utils::shift_letter;

pub fn encrypt(message: &String, key: &String) -> String {
    println!("Encrypting with Shift, message: {}, key: {}", message, key);

    let key = match key.parse::<i32>() {
        Ok(k) => k,
        Err(_message) => return format!("Invalid key for shift cipher: {}", key)
    };

    shift_string(&message.to_lowercase(), key)
}

pub fn decrypt(message: &String, key: &String) -> String {
    println!("Decrypting with Shift, message: {}, key: {}", message, key);

    let key = match key.parse::<i32>() {
        Ok(k) => k,
        Err(_message) => return format!("Invalid key for shift cipher: {}", key)
    };

    shift_string(&message.to_lowercase(), -key)
}

fn shift_string(message: &String, offset: i32) -> String {
    let mut ciphertext = String::new();

    for letter in message.chars() {
        if letter.is_alphabetic() {
            let shifted_letter = shift_letter(letter, offset);
            ciphertext = ciphertext.add(shifted_letter.to_string().as_str());
        } else {
            ciphertext = ciphertext.add(letter.to_string().as_str());
        }
    }

    ciphertext
}
