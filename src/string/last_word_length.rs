/// Counts Unicode scalar values in the final whitespace-separated word.
///
/// Returns zero for empty or whitespace-only text. This is a character count,
/// not a byte count or a count of displayed grapheme clusters.
/// Adapted from LeetCode 58, Length of Last Word.
///
/// ```
/// use useful_functions::string::last_word_length;
/// assert_eq!(last_word_length("  fly me to the moon  "), 4);
/// assert_eq!(last_word_length("hello café"), 4);
/// ```
pub fn last_word_length(text: &str) -> usize {
    text.split_whitespace()
        .next_back()
        .map_or(0, |word| word.chars().count())
}
