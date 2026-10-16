/// Increments a nonnegative decimal integer represented by most-significant-first digits.
///
/// Supports numbers too large for built-in integer types. Empty input represents
/// zero, leading zeros are preserved, and any digit greater than 9 returns `None`.
/// The input is not modified.
/// Adapted from LeetCode 66, Plus One.
///
/// ```
/// use useful_functions::number::increment_decimal_digits;
/// assert_eq!(increment_decimal_digits(&[9, 9]), Some(vec![1, 0, 0]));
/// assert_eq!(increment_decimal_digits(&[12]), None);
/// ```
pub fn increment_decimal_digits(digits: &[u8]) -> Option<Vec<u8>> {
    if digits.iter().any(|&digit| digit > 9) {
        return None;
    }

    let mut output = digits.to_vec();
    for digit in output.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return Some(output);
        }
        *digit = 0;
    }

    output.insert(0, 1);
    Some(output)
}
