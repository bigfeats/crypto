use super::multiply;
use super::shift;

pub fn encrypt(message: &str, key: &str) -> Result<String, String> {
    let (mult_key, shift_key) = split_keys(key)?;

    let mult_cipher = multiply::encrypt(message, mult_key.as_str())?;
    shift::encrypt(mult_cipher.as_str(), shift_key.as_str())
}

pub fn decrypt(message: &str, key: &str) -> Result<String, String> {
    let (mult_key, shift_key) = split_keys(key)?;

    let shift_plaintext = shift::decrypt(message, shift_key.as_str())?;
    multiply::decrypt(shift_plaintext.as_str(), mult_key.as_str())
}

fn split_keys(key: &str) -> Result<(String, String), String> {
    let keys: Vec<&str> = key.split(",").collect();

    if keys.len() == 2 {
        Ok((keys[0].trim().to_string(), keys[1].trim().to_string()))
    } else {
        Err(format!("Invalid keys: {}, expected two comma separated keys", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_keys() {
        let (mult_key, shift_key) = split_keys("1,2").unwrap();

        assert_eq!(mult_key, "1");
        assert_eq!(shift_key, "2");

        let (mult_key, shift_key) = split_keys("3, 4").unwrap();

        assert_eq!(mult_key, "3");
        assert_eq!(shift_key, "4");
    }

    #[test]
    fn no_op() {
        assert_eq!("hello", encrypt("hello", "1,0").unwrap());
        assert_eq!("hello", decrypt("hello", "1,0").unwrap());
    }

    #[test]
    fn converts_to_lowercase() {
        assert_eq!("hello", encrypt("HeLlO", "1,0").unwrap());
    }

    #[test]
    fn does_not_change_non_alphabetic_characters() {
        assert_eq!("hi!  this tests the handling of non-alphabetic characters!@#$%^&*()", encrypt("Hi!  This tests the handling of non-alphabetic characters!@#$%^&*()", "1,0").unwrap());
    }

    #[test]
    fn invalid_key_error() {
        let error = encrypt("abcd", "13,0").unwrap_err();
        assert_eq!("Invalid key: 13", error);

        let error = decrypt("abcd", "13,0").unwrap_err();
        assert_eq!("Invalid key: 13", error);
    }

    #[test]
    fn can_encrypt_and_decrypt() {
        // Message purposefully lacks uppercase letters, since this causes a purposeful difference.
        let message = "this should match, even after being encrypted and decrypted!";

        for mult_key in [3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25].iter() {
            for shift_key in 1..100 {
                let key_str = format!("{},{}", mult_key, shift_key);

                let cipher = encrypt(message, &key_str).unwrap();
                let decrypted = decrypt(&cipher, &key_str).unwrap();

                assert_ne!(cipher, message);
                assert_eq!(decrypted, message);
            }
        }
    }
}
