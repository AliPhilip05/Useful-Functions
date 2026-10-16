//! Searching and placement utilities for slices.

mod can_insert_nonadjacent;
mod find_equal_range;
mod find_index_matching_digit_sum;
mod find_pair_with_sum;

pub use can_insert_nonadjacent::can_insert_nonadjacent;
pub use find_equal_range::find_equal_range;
pub use find_index_matching_digit_sum::find_index_matching_digit_sum;
pub use find_pair_with_sum::find_pair_with_sum;
