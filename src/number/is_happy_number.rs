/// Checks whether repeated squared-digit sums eventually reach one.
///
/// Returns false for zero and negative integers. Cycle detection uses constant
/// extra space.
/// Adapted from LeetCode 202, Happy Number.
///
/// ```
/// use useful_functions::number::is_happy_number;
/// assert!(is_happy_number(19));
/// assert!(!is_happy_number(2));
/// ```
pub fn is_happy_number(value: i32) -> bool {
    if value <= 0 {
        return false;
    }

    let mut slow = value;
    let mut fast = value;
    loop {
        slow = digit_square_sum(slow);
        fast = digit_square_sum(digit_square_sum(fast));
        if fast == 1 {
            return true;
        }
        if slow == fast {
            return false;
        }
    }
}

fn digit_square_sum(mut value: i32) -> i32 {
    let mut total = 0;
    while value > 0 {
        let digit = value % 10;
        total += digit * digit;
        value /= 10;
    }
    total
}
