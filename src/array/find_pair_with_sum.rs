use std::collections::HashMap;

/// Finds two distinct indices whose values sum to `target`.
///
/// Returns the first pair encountered while scanning from left to right, or
/// `None` if no pair exists. The smaller index is returned first. Arithmetic
/// that would exceed the i32 range is excluded rather than wrapped.
/// Takes O(n) expected time and O(n) extra space.
/// Adapted from LeetCode 1, Two Sum.
///
/// ```
/// use useful_functions::array::find_pair_with_sum;
/// assert_eq!(find_pair_with_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
/// assert_eq!(find_pair_with_sum(&[1, 2], 10), None);
/// ```
pub fn find_pair_with_sum(values: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen = HashMap::new();
    for (index, &value) in values.iter().enumerate() {
        if let Some(complement) = target.checked_sub(value) {
            if let Some(&previous_index) = seen.get(&complement) {
                return Some((previous_index, index));
            }
        }
        seen.entry(value).or_insert(index);
    }

    None
}
