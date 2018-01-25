use algorithms::utils;

pub fn encrypt(message: &str, key: &str) -> Result<String, String> {
    let key = utils::parse_integer_key(key)?;

    // Calculate the inverse key to cause a bad key to fail up front.
    calculate_inverse_key(key)?;

    Ok(multiply_string(&message.to_lowercase(), key as u16))
}

pub fn decrypt(message: &str, key: &str) -> Result<String, String> {
    let key = utils::parse_integer_key(key)?;
    let inverse_key = calculate_inverse_key(key)?;

    Ok(multiply_string(&message.to_lowercase(), inverse_key))
}

fn calculate_inverse_key(key: i32) -> Result<u16, String> {
    match utils::math::calculate_multiplicative_inverse(key as u16, 26u16) {
        Ok(k) => Ok(k),
        Err(_m) => Err(format!("Invalid key: {}", key))
    }
}

fn multiply_string(message: &str, mult: u16) -> String {
    let mut ciphertext = String::new();

    for letter in message.chars() {
        if letter.is_alphabetic() {
            let shifted_letter = multiply_letter(letter, mult);
            ciphertext.push(shifted_letter);
        } else {
            ciphertext.push(letter);
        }
    }

    ciphertext
}

fn multiply_letter(plain_letter: char, mult: u16) -> char {
    let letter_index = utils::get_letter_index(plain_letter);
    let offset_index = letter_index as u16 * mult;

    utils::get_letter(offset_index as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_key_no_op() {
        assert_eq!("hello", encrypt("hello", "1").unwrap());
        assert_eq!("hello", decrypt("hello", "1").unwrap());
    }

    #[test]
    fn converts_to_lowercase() {
        assert_eq!("hello", encrypt("HeLlO", "1").unwrap());
    }

    #[test]
    fn does_not_change_non_alphabetic_characters() {
        assert_eq!("hi!  this tests the handling of non-alphabetic characters!@#$%^&*()", encrypt("Hi!  This tests the handling of non-alphabetic characters!@#$%^&*()", "1").unwrap());
    }

    #[test]
    fn invalid_key_error() {
        let error = encrypt("abcd", "13").unwrap_err();
        assert_eq!("Invalid key: 13", error);

        let error = decrypt("abcd", "13").unwrap_err();
        assert_eq!("Invalid key: 13", error);
    }

    #[test]
    fn can_encrypt_and_decrypt() {
        // Message purposefully lacks uppercase letters, since this causes a purposeful difference.
        let message = "this should match, even after being encrypted and decrypted!";

        for key in [3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25].iter() {
            let key_str = &key.to_string();
            let cipher = encrypt(message, key_str).unwrap();
            let decrypted = decrypt(&cipher, key_str).unwrap();

            assert_ne!(cipher, message);
            assert_eq!(decrypted, message);
        }
    }
}
