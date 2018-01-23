pub mod math;

use self::math::modulus;

pub fn shift_letter(plain_letter: char, offset: i32) -> char {
    let base = 'a' as u8;
    let as_number = plain_letter as u8 - base;
    let offset_number = as_number as i32 + offset;

    let mod_offset = modulus(offset_number, 26) as u8;
    let shifted_char = (base + mod_offset) as char;

    shifted_char
}
