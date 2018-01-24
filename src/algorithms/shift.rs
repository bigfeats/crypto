use std::ops::Add; // For adding to a String
use algorithms::utils;

pub fn encrypt(message: &str, key: &str) -> Result<String, String> {
    let parsed_key = utils::parse_integer_key(key)?;
    let ciphertext = shift_string(&message.to_lowercase(), parsed_key);

    Ok(ciphertext)
}

pub fn decrypt(message: &str, key: &str) -> Result<String, String> {
    let parsed_key = utils::parse_integer_key(key)?;
    let plaintext = shift_string(&message.to_lowercase(), -parsed_key);

    Ok(plaintext)
}

fn shift_string(message: &str, offset: i32) -> String {
    let mut ciphertext = String::new();

    for letter in message.chars() {
        if letter.is_alphabetic() {
            let shifted_letter = utils::shift_letter(letter, offset);
            ciphertext = ciphertext.add(shifted_letter.to_string().as_str());
        } else {
            ciphertext = ciphertext.add(letter.to_string().as_str());
        }
    }

    ciphertext
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_key_no_op() {
        assert_eq!("hello", encrypt("hello", "0").unwrap());
        assert_eq!("hello", decrypt("hello", "0").unwrap());
    }

    #[test]
    fn converts_to_lowercase() {
        assert_eq!("hello", encrypt("HeLlO", "0").unwrap());
    }

    #[test]
    fn does_not_change_non_alphabetic_characters() {
        assert_eq!("hi!  this tests the handling of non-alphabetic characters!@#$%^&*()", encrypt("Hi!  This tests the handling of non-alphabetic characters!@#$%^&*()", "0").unwrap());
    }

    #[test]
    fn can_encrypt_and_decrypt() {
        // Message purposefully lacks uppercase letters, since this causes a purposeful difference.
        let message = "this should match, even after being encrypted and decrypted!";

        for key in 0..500 {
            let key_str = &key.to_string();
            let cipher = encrypt(message, key_str).unwrap();
            let decrypted = decrypt(&cipher, key_str).unwrap();

            // cipher and message should match if the key is divisible by 26
            if key % 26 != 0 {
                assert_ne!(cipher, message);
            } else {
                assert_eq!(cipher, message);
            }

            assert_eq!(decrypted, message);
        }
    }
}
