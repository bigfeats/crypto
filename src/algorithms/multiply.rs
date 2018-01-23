pub fn encrypt(message: &String, key: &String) -> String {
    println!("Encrypting with multiply cipher, message: {}, key: {}", message, key);
    String::from("hi i'm encrypted")
}

pub fn decrypt(message: &String, key: &String) -> String {
    println!("Decrypting with multiply cipher, message: {}, key: {}", message, key);
    String::from("hi i'm decrypted")
}
