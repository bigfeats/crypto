// % is a remainder function, not a modulus.  They work differently for negative numbers.
pub fn modulus(value: i32, modulus: u16) -> u16 {

    // The modulus must be positive, but treat it as i32 for compatibility with a negative `value`.
    let modulus = modulus as i32;

    (((value % modulus) + modulus) % modulus) as u16
}

pub fn calculate_multiplicative_inverse(value: u16, modulus: u16) -> Result<u16, String> {
    // Algorithm adapted from: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    let value = value as i32;
    let modulus = modulus as i32;
    let (mut t, mut new_t) = (0i32, 1i32);
    let (mut r, mut new_r) = (modulus, value);

    while new_r != 0 {
        let quotient = r / new_r;

        let temp_t = new_t;
        new_t = t - quotient * new_t;
        t = temp_t;

        let temp_r = new_r;
        new_r = r - quotient * new_r;
        r = temp_r;
    }

    if r > 1 {
        return Err(format!("No inverse for value: {}, modulus: {}", value, modulus))
    }

    if t < 0 {
        t = t + modulus
    }

    return Ok(t as u16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_case() {
        let modulus = 26;
        let multiplier = 3;
        let inverse = calculate_multiplicative_inverse(multiplier, modulus).unwrap();

        assert_eq!(9, inverse);

        let test_value = 12;
        let product = (test_value * multiplier) % modulus;
        let inversed_product = (product * inverse) % modulus;

        assert_eq!(test_value, inversed_product);
    }

    #[test]
    fn test_invalid_case() {
        assert_eq!("No inverse for value: 2, modulus: 26", calculate_multiplicative_inverse(2, 26).unwrap_err());
    }

    #[test]
    fn check_inversion() {
        let modulus = 26;

        for multiplier in [3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25].iter() {
            let inverse = calculate_multiplicative_inverse(*multiplier, modulus).unwrap();

            for value in 1..26 {
                let product = (value * multiplier) % modulus;
                let inversed_product = (product * inverse) % modulus;

                assert_eq!(value, inversed_product);
            }
        }
    }
}
