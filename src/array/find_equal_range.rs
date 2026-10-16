/// Finds the inclusive first and last indices of `target` in a sorted slice.
///
/// The slice must be sorted in ascending order. Returns `None` if absent.
/// Uses two binary searches, taking O(log n) time and O(1) extra space.
/// Adapted from LeetCode 34, Find First and Last Position of Element in Sorted Array.
///
/// ```
/// use useful_functions::array::find_equal_range;
/// assert_eq!(find_equal_range(&[5, 7, 7, 8, 8, 10], 8), Some((3, 4)));
/// assert_eq!(find_equal_range(&[5, 7, 7, 8, 8, 10], 6), None);
/// ```
pub fn find_equal_range(values: &[i32], target: i32) -> Option<(usize, usize)> {
    let first = values.partition_point(|&value| value < target);
    if values.get(first) != Some(&target) {
        return None;
    }

    let end = values.partition_point(|&value| value <= target);
    Some((first, end - 1))
}
