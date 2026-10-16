use useful_functions::{array::*, number::*, string::*};

#[test]
fn string_functions_support_empty_inputs() {
    assert_eq!(common_repeating_pattern("", "ABC"), "");
    assert_eq!(common_repeating_pattern("ABC", ""), "");
    assert_eq!(interleave_characters("", ""), "");
    assert_eq!(interleave_characters("", "abc"), "abc");
    assert_eq!(interleave_characters("abc", ""), "abc");
    assert_eq!(last_word_length(""), 0);
    assert_eq!(reverse_vowels(""), "");
    assert_eq!(reverse_word_order(""), "");
}

#[test]
fn common_patterns_must_repeat_to_fill_both_inputs() {
    assert_eq!(common_repeating_pattern("ABCABC", "ABC"), "ABC");
    assert_eq!(common_repeating_pattern("ABABAB", "ABAB"), "AB");
    assert_eq!(common_repeating_pattern("AAAAAB", "AAA"), "");
    assert_eq!(common_repeating_pattern("é🦀é🦀", "é🦀"), "é🦀");
}

#[test]
fn interleaving_appends_the_longer_tail_without_splitting_utf8() {
    assert_eq!(interleave_characters("abc", "pqr"), "apbqcr");
    assert_eq!(interleave_characters("ab", "pqrs"), "apbqrs");
    assert_eq!(interleave_characters("abcd", "pq"), "apbqcd");
    assert_eq!(interleave_characters("é🦀", "你好!"), "é你🦀好!");
}

#[test]
fn words_use_whitespace_and_count_characters() {
    assert_eq!(last_word_length("   fly me   to   the moon  "), 4);
    assert_eq!(last_word_length("hello\tcafé\n"), 4);
    assert_eq!(last_word_length(" \t\n"), 0);
    assert_eq!(reverse_word_order(" a good   example "), "example good a");
    assert_eq!(reverse_word_order("你好\t世界\n"), "世界 你好");
    assert_eq!(reverse_word_order(" \t\n"), "");
}

#[test]
fn reversing_vowels_preserves_other_characters() {
    assert_eq!(reverse_vowels("leetcode"), "leotcede");
    assert_eq!(reverse_vowels("aA"), "Aa");
    assert_eq!(reverse_vowels("éa🦀E"), "éE🦀a");
    assert_eq!(reverse_vowels("rhythm"), "rhythm");
}

#[test]
fn pair_search_handles_duplicates_missing_values_and_overflow() {
    assert_eq!(find_pair_with_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(find_pair_with_sum(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(find_pair_with_sum(&[3, 3], 6), Some((0, 1)));
    assert_eq!(find_pair_with_sum(&[-3, 4, 3], 0), Some((0, 2)));
    assert_eq!(find_pair_with_sum(&[], 0), None);
    assert_eq!(find_pair_with_sum(&[3], 6), None);
    assert_eq!(find_pair_with_sum(&[i32::MAX, 1], i32::MIN), None);
    assert_eq!(find_pair_with_sum(&[i32::MIN, i32::MAX], -1), Some((0, 1)));
}

#[test]
fn range_search_handles_boundaries_and_duplicates() {
    assert_eq!(find_equal_range(&[5, 7, 7, 8, 8, 10], 8), Some((3, 4)));
    assert_eq!(find_equal_range(&[2, 2, 2], 2), Some((0, 2)));
    assert_eq!(find_equal_range(&[1], 1), Some((0, 0)));
    assert_eq!(find_equal_range(&[], 0), None);
    for target in [-1, 0, 2, 4, 5] {
        assert_eq!(find_equal_range(&[1, 3], target), None);
    }
}

#[test]
fn digit_sum_search_returns_the_first_match() {
    assert_eq!(find_index_matching_digit_sum(&[1, 3, 2]), Some(2));
    assert_eq!(find_index_matching_digit_sum(&[1, 10, 11]), Some(1));
    assert_eq!(find_index_matching_digit_sum(&[0]), Some(0));
    assert_eq!(find_index_matching_digit_sum(&[1, 2, 3]), None);
    assert_eq!(find_index_matching_digit_sum(&[]), None);
}

#[test]
fn placement_handles_small_layouts_and_invalid_adjacency() {
    assert!(can_insert_nonadjacent(&[], 0));
    assert!(!can_insert_nonadjacent(&[], 1));
    assert!(can_insert_nonadjacent(&[false], 1));
    assert!(!can_insert_nonadjacent(&[true], 1));
    assert!(can_insert_nonadjacent(
        &[true, false, false, false, true],
        1
    ));
    assert!(!can_insert_nonadjacent(
        &[true, false, false, false, true],
        2
    ));
    assert!(can_insert_nonadjacent(&[false, false, false], 2));
    assert!(!can_insert_nonadjacent(&[true, true], 0));
}

#[test]
fn placement_matches_exhaustive_search_for_small_layouts() {
    // Compare the greedy routine to every possible final layout.
    for length in 0..=7 {
        for original in 0usize..(1 << length) {
            let occupied: Vec<bool> = (0..length).map(|i| original & (1 << i) != 0).collect();
            for count in 0..=length + 1 {
                let expected = (0usize..(1 << length)).any(|candidate| {
                    candidate & original == original
                        && candidate & (candidate << 1) == 0
                        && (candidate.count_ones() - original.count_ones()) as usize >= count
                });
                assert_eq!(can_insert_nonadjacent(&occupied, count), expected);
            }
        }
    }
}

#[test]
fn decimal_increment_handles_carry_empty_and_invalid_digits() {
    assert_eq!(increment_decimal_digits(&[1, 2, 3]), Some(vec![1, 2, 4]));
    assert_eq!(increment_decimal_digits(&[1, 9, 9]), Some(vec![2, 0, 0]));
    assert_eq!(increment_decimal_digits(&[9, 9]), Some(vec![1, 0, 0]));
    assert_eq!(increment_decimal_digits(&[]), Some(vec![1]));
    assert_eq!(increment_decimal_digits(&[0, 0]), Some(vec![0, 1]));
    assert_eq!(increment_decimal_digits(&[1, 10]), None);
    assert_eq!(
        increment_decimal_digits(&vec![9; 1000]),
        Some([vec![1], vec![0; 1000]].concat())
    );
}

#[test]
fn palindrome_checks_handle_signs_zero_and_large_values() {
    for value in [0, 1, 11, 121, 1221, 12321, 2_147_447_412] {
        assert!(is_decimal_palindrome(value), "{value}");
    }
    for value in [-121, i32::MIN, 10, 123, i32::MAX] {
        assert!(!is_decimal_palindrome(value), "{value}");
    }
}

#[test]
fn happy_numbers_terminate_for_cycles_and_nonpositive_inputs() {
    for value in [1, 7, 19, 100] {
        assert!(is_happy_number(value));
    }
    for value in [i32::MIN, -19, 0, 2, 4, i32::MAX] {
        assert!(!is_happy_number(value));
    }
}
