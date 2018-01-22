pub fn encrypt(message: &String, key: &String) -> String {
    println!("Encrypting with Hill, message: {}, key: {}", message, key);
    String::from("hi i'm encrypted")
}

pub fn decrypt(message: &String, key: &String) -> String {
    println!("Decrypting with Hill, message: {}, key: {}", message, key);
    String::from("hi i'm decrypted")
}
