// % is a remainder function, not a modulus.  They work differently for negative numbers.
pub fn modulus(value: i32, modulus: u16) -> u16 {

    // The modulus must be positive, but treat it as i32 for compatibility with a negative `value`.
    let modulus = modulus as i32;

    (((value % modulus) + modulus) % modulus) as u16
}
