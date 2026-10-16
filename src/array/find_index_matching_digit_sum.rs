/// Finds the first index equal to the decimal digit sum of its value.
///
/// Takes nonnegative integers and returns `None` if there is no matching index.
/// The digit sum of zero is zero.
/// Adapted from LeetCode 3550, Smallest Index With Digit Sum Equal to Index.
///
/// ```
/// use useful_functions::array::find_index_matching_digit_sum;
/// assert_eq!(find_index_matching_digit_sum(&[1, 10, 11]), Some(1));
/// ```
pub fn find_index_matching_digit_sum(values: &[u32]) -> Option<usize> {
    values.iter().enumerate().find_map(|(index, &value)| {
        let mut value = value;
        let mut sum = 0;
        while value > 0 {
            sum += value % 10;
            value /= 10;
        }

        (sum as usize == index).then_some(index)
    })
}
