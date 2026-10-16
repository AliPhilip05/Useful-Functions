/// Reverses the positions of English vowels, preserving all other characters.
///
/// Both uppercase and lowercase `a`, `e`, `i`, `o`, and `u` are vowels here;
/// accented letters and `y` are left in place.
/// Adapted from LeetCode 345, Reverse Vowels of a String.
///
/// ```
/// use useful_functions::string::reverse_vowels;
/// assert_eq!(reverse_vowels("hello"), "holle");
/// ```
pub fn reverse_vowels(text: &str) -> String {
    let is_vowel = |character: char| {
        matches!(
            character,
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
        )
    };
    let mut vowels = text.chars().rev().filter(|&character| is_vowel(character));

    text.chars()
        .map(|character| {
            if is_vowel(character) {
                vowels.next().unwrap_or(character)
            } else {
                character
            }
        })
        .collect()
}
