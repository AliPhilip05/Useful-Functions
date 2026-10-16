/// Reverses whitespace-separated words and joins them with single spaces.
///
/// Leading and trailing whitespace is removed. Empty or whitespace-only text
/// produces an empty string.
/// Adapted from LeetCode 151, Reverse Words in a String.
///
/// ```
/// use useful_functions::string::reverse_word_order;
/// assert_eq!(reverse_word_order("  the sky  is blue "), "blue is sky the");
/// ```
pub fn reverse_word_order(text: &str) -> String {
    text.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}
