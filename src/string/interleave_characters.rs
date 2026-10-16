/// Alternates characters from two strings, starting with `first`.
///
/// Any remaining characters from the longer input are appended unchanged.
/// Adapted from LeetCode 1768, Merge Strings Alternately.
///
/// ```
/// use useful_functions::string::interleave_characters;
/// assert_eq!(interleave_characters("ab", "pqrs"), "apbqrs");
/// ```
pub fn interleave_characters(first: &str, second: &str) -> String {
    let mut output = String::with_capacity(first.len() + second.len());
    let mut first = first.chars();
    let mut second = second.chars();

    loop {
        match (first.next(), second.next()) {
            (Some(left), Some(right)) => {
                output.push(left);
                output.push(right);
            }
            (Some(left), None) => {
                output.push(left);
                output.extend(first);
                break;
            }
            (None, Some(right)) => {
                output.push(right);
                output.extend(second);
                break;
            }
            (None, None) => break,
        }
    }

    output
}
