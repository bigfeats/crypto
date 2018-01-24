pub mod affine;
pub mod hill;
pub mod multiply;
pub mod shift;
pub mod utils;

pub fn encrypt(requested_algorithm: &String, message: &String, key: &String) -> Result<String, String> {
    match requested_algorithm.as_str() {
        "affine" => affine::encrypt(message, key),
        "hill" => hill::encrypt(message, key),
        "multiply" => multiply::encrypt(message, key),
        "shift" => shift::encrypt(message, key),
        _ => Err(format!("Unsupported algorithm: {}", requested_algorithm))
    }
}

pub fn decrypt(requested_algorithm: &String, message: &String, key: &String) -> Result<String, String> {
    match requested_algorithm.as_str() {
        "affine" => affine::decrypt(message, key),
        "hill" => hill::decrypt(message, key),
        "multiply" => multiply::decrypt(message, key),
        "shift" => shift::decrypt(message, key),
        _ => Err(format!("Unsupported algorithm: {}", requested_algorithm))
    }
}
