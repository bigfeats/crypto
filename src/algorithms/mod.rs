pub mod hill;
pub mod shift;

pub fn encrypt(requested_algorithm: &String, message: &String, key: &String) -> String {
    match requested_algorithm.as_str() {
        "hill" => hill::encrypt(message, key),
        "shift" => shift::encrypt(message, key),
        _ => format!("ERROR: Unsupported algorithm: {}", requested_algorithm)
    }
}

pub fn decrypt(requested_algorithm: &String, message: &String, key: &String) -> String {
    match requested_algorithm.as_str() {
        "hill" => hill::decrypt(message, key),
        "shift" => shift::decrypt(message, key),
        _ => format!("ERROR: Unsupported algorithm: {}", requested_algorithm)
    }
}
