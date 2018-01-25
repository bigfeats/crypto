pub mod math;

use self::math::modulus;

// Assumes a lower case letter, index starts at 1.
pub fn get_letter_index(letter: char) -> u8 {
    letter as u8 - 'a' as u8 + 1u8
}

// Handles modulus when index > alphabet size
pub fn get_letter(index: i32) -> char {
    let mod_offset = modulus(index - 1, 26) as u8;
    ('a' as u8 + mod_offset) as char
}

pub fn parse_integer_key(key: &str) -> Result<i32, String> {
    match key.parse::<i32>() {
        Ok(k) => Ok(k),
        Err(_message) => return Err(format!("Invalid key, expected integer: {}", key))
    }
}

