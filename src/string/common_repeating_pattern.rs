/// Returns the longest pattern whose repetitions produce both input strings.
///
/// Returns an empty string if either input is empty or no shared pattern exists.
/// Adapted from LeetCode 1071, Greatest Common Divisor of Strings.
///
/// ```
/// use useful_functions::string::common_repeating_pattern;
/// assert_eq!(common_repeating_pattern("ABABAB", "ABAB"), "AB");
/// assert_eq!(common_repeating_pattern("hello", "world"), "");
/// ```
pub fn common_repeating_pattern(first: &str, second: &str) -> String {
    if first.is_empty()
        || second.is_empty()
        || first
            .chars()
            .chain(second.chars())
            .ne(second.chars().chain(first.chars()))
    {
        return String::new();
    }

    let mut first_length = first.chars().count();
    let mut second_length = second.chars().count();
    while second_length != 0 {
        (first_length, second_length) = (second_length, first_length % second_length);
    }

    first.chars().take(first_length).collect()
}
