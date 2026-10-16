/// Checks whether an integer reads the same forwards and backwards in decimal.
///
/// Negative integers are not palindromes. Zero is a palindrome.
/// Reverses only half the digits to avoid integer overflow.
/// Adapted from LeetCode 9, Palindrome Number.
///
/// ```
/// use useful_functions::number::is_decimal_palindrome;
/// assert!(is_decimal_palindrome(12321));
/// assert!(!is_decimal_palindrome(-121));
/// ```
pub fn is_decimal_palindrome(mut value: i32) -> bool {
    if value < 0 || (value != 0 && value % 10 == 0) {
        return false;
    }

    let mut reversed_half = 0;
    while value > reversed_half {
        reversed_half = reversed_half * 10 + value % 10;
        value /= 10;
    }

    value == reversed_half || value == reversed_half / 10
}
