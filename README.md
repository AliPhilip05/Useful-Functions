# Useful Functions

Handwritten LeetCode solutions organized as a practical reference library.
The goal is to make the functions from my practice easy to find and reuse by
grouping them by the kind of data they work with and naming files after their
purpose.

The collection contains **12 Rust solutions and one Python solution**. The source
code is preserved as originally written, including function names, comments,
and LeetCode's `Solution` wrappers. The organization and documentation make the
collection easier to navigate while keeping the implementations handwritten.

## Organization

```text
src/
  string/          # String manipulation and repeating patterns (Rust)
  array/           # Array searches and nonadjacent placement (Rust)
  number/          # Decimal digits and integer checks (Rust)
useful_functions/
  ranges/          # Missing integer ranges (Python)
```

## Strings

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`gcd_of_strings`](src/string/common_repeating_pattern.rs) | Finds the longest shared repeating text pattern | 1071 · Greatest Common Divisor of Strings |
| [`merge_alternately`](src/string/interleave_characters.rs) | Alternates characters and appends any remaining tail | 1768 · Merge Strings Alternately |
| [`length_of_last_word`](src/string/last_word_length.rs) | Finds the length of the final word | 58 · Length of Last Word |
| [`reverse_vowels`](src/string/reverse_vowels.rs) | Reverses English vowels while preserving other characters | 345 · Reverse Vowels of a String |
| [`reverse_words`](src/string/reverse_word_order.rs) | Reverses the order of words | 151 · Reverse Words in a String |

## Arrays

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`can_place_flowers`](src/array/can_insert_nonadjacent.rs) | Checks capacity for new flowers without adjacent planting | 605 · Can Place Flowers |
| [`search_range`](src/array/find_equal_range.rs) | Finds the first and last matching indices in a sorted array | 34 · Find First and Last Position of Element in Sorted Array |
| [`smallest_index`](src/array/find_index_matching_digit_sum.rs) | Finds the first index equal to its value's digit sum | 3550 · Smallest Index With Digit Sum Equal to Index |
| [`two_sum`](src/array/find_pair_with_sum.rs) | Finds two distinct indices whose values add to a target | 1 · Two Sum |

## Numbers

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`plus_one`](src/number/increment_decimal_digits.rs) | Adds one to an integer represented by decimal digits | 66 · Plus One |
| [`is_palindrome`](src/number/is_decimal_palindrome.rs) | Checks whether an integer's decimal digits are symmetric | 9 · Palindrome Number |
| [`is_happy`](src/number/is_happy_number.rs) | Checks whether repeated squared-digit sums reach one | 202 · Happy Number |

## Ranges

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`findMissingRanges`](useful_functions/ranges/find_missing_ranges.py) | Finds inclusive gaps within integer bounds from sorted, unique values | 163 · Missing Ranges |

## Using the collection

Use the catalog to find an algorithm, then read its source and original problem
constraints before bringing it into your project. These are standalone practice
solutions with their original interfaces and assumptions. The Rust files use
`impl Solution`, and the Python file defines a `Solution` class.

When reusing a function, adapt its wrapper and inputs to your application and
check it against your own requirements. The repository is a collection of source
files rather than a packaged Rust crate or Python distribution.

New additions should be handwritten, live in the appropriate category, and have
a filename that describes their purpose. Preserve the original implementation
when organizing existing solutions.
