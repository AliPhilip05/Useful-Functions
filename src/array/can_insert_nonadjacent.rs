/// Checks whether `count` new occupied slots can be added without adjacency.
///
/// `true` marks an occupied slot; `false` marks an empty slot. The input is not
/// modified. Returns false if the existing layout already has adjacent occupied
/// slots. An empty layout accepts zero insertions only.
/// Adapted from LeetCode 605, Can Place Flowers.
///
/// ```
/// use useful_functions::array::can_insert_nonadjacent;
/// assert!(can_insert_nonadjacent(&[true, false, false, false, true], 1));
/// assert!(!can_insert_nonadjacent(&[true, false, false, false, true], 2));
/// ```
pub fn can_insert_nonadjacent(occupied: &[bool], count: usize) -> bool {
    if occupied.windows(2).any(|pair| pair[0] && pair[1]) {
        return false;
    }

    let mut available = 0;
    let mut previous_occupied = false;
    for (index, &current_occupied) in occupied.iter().enumerate() {
        let next_occupied = occupied.get(index + 1).copied().unwrap_or(false);
        if !previous_occupied && !current_occupied && !next_occupied {
            available += 1;
            previous_occupied = true;
        } else {
            previous_occupied = current_occupied;
        }
    }

    available >= count
}
