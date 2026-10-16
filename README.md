# Useful Functions

LeetCode problems turned into a practical library. This repository takes the
algorithms from my practice solutions and gives them descriptive names, reusable
interfaces, and a home organized by the type of data they work with.

The collection currently contains **12 Rust functions and one Python function**.
Each function lives in a file named after its behavior, with documentation,
examples, and tests. Rust utilities are ordinary public functions that borrow
their inputs; Python utilities are importable through `useful_functions`.

## Organization

```text
src/
  string/          # Text manipulation and repeating patterns (Rust)
  array/           # Slice searches and nonadjacent placement (Rust)
  number/          # Decimal digits and integer predicates (Rust)
useful_functions/
  ranges/          # Missing integer ranges (Python)
tests/             # Rust and Python behavior checks
```

## Rust

Add the library to your project's `Cargo.toml`:

```toml
[dependencies]
useful-functions = { git = "https://github.com/AliPhilip05/Useful-Functions.git" }
```

```rust
use useful_functions::{array, number, string};

assert_eq!(string::reverse_word_order("  the sky is blue  "), "blue is sky the");
assert_eq!(string::interleave_characters("ab", "pqrs"), "apbqrs");
assert_eq!(array::find_pair_with_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
assert_eq!(number::increment_decimal_digits(&[9, 9]), Some(vec![1, 0, 0]));
```

Search functions return `Option` instead of sentinel values such as `-1`.
Text functions support UTF-8 and operate on Unicode scalar values, rather than
displayed grapheme clusters. Word functions recognize Unicode whitespace;
`reverse_vowels` recognizes English vowels only. Function documentation describes
input requirements and behavior for empty or invalid inputs.

### Strings

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`common_repeating_pattern`](src/string/common_repeating_pattern.rs) | Finds the longest shared repeating text pattern | 1071 · Greatest Common Divisor of Strings |
| [`interleave_characters`](src/string/interleave_characters.rs) | Alternates characters and appends any remaining tail | 1768 · Merge Strings Alternately |
| [`last_word_length`](src/string/last_word_length.rs) | Counts characters in the final word | 58 · Length of Last Word |
| [`reverse_vowels`](src/string/reverse_vowels.rs) | Reverses English vowels while preserving other characters | 345 · Reverse Vowels of a String |
| [`reverse_word_order`](src/string/reverse_word_order.rs) | Reverses words and normalizes whitespace | 151 · Reverse Words in a String |

### Arrays

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`can_insert_nonadjacent`](src/array/can_insert_nonadjacent.rs) | Checks capacity for new occupied slots without adjacency | 605 · Can Place Flowers |
| [`find_equal_range`](src/array/find_equal_range.rs) | Finds the inclusive first and last matching indices in a sorted slice | 34 · Find First and Last Position of Element in Sorted Array |
| [`find_index_matching_digit_sum`](src/array/find_index_matching_digit_sum.rs) | Finds the first index equal to its value's digit sum | 3550 · Smallest Index With Digit Sum Equal to Index |
| [`find_pair_with_sum`](src/array/find_pair_with_sum.rs) | Finds two distinct indices whose values add to a target | 1 · Two Sum |

### Numbers

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`increment_decimal_digits`](src/number/increment_decimal_digits.rs) | Adds one to a decimal digit sequence of arbitrary length | 66 · Plus One |
| [`is_decimal_palindrome`](src/number/is_decimal_palindrome.rs) | Checks whether an integer's decimal digits are symmetric | 9 · Palindrome Number |
| [`is_happy_number`](src/number/is_happy_number.rs) | Checks whether repeated squared-digit sums reach one | 202 · Happy Number |

## Python

From a clone of the repository, install the Python package with `python -m pip install .`,
or import it directly while working in the repository root:

```python
from useful_functions.ranges import find_missing_ranges

assert find_missing_ranges([0, 1, 3, 50, 75], 0, 99) == [
    [2, 2], [4, 49], [51, 74], [76, 99]
]
```

| Function | What it does | LeetCode origin |
| --- | --- | --- |
| [`find_missing_ranges`](useful_functions/ranges/find_missing_ranges.py) | Finds inclusive gaps within integer bounds from sorted, unique values | 163 · Missing Ranges |

Python requires version 3.9 or newer. The range utility accepts any iterable of
sorted, unique integers within the supplied bounds and raises `ValueError` for
invalid bounds, ordering, duplicates, or out-of-range values. It has no runtime
dependencies.

## Development

The Rust crate has no external dependencies. Run the checks from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
python3 -m unittest discover -s tests -p 'test_*.py' -v
python3 -m doctest useful_functions/ranges/find_missing_ranges.py
```

Use `cargo doc --open` to browse the Rust API documentation. GitHub Actions runs
formatting, lint, Rust example and integration tests, and Python tests on pushes
and pull requests.

When adding a practice solution, choose a name that explains its behavior, place
it in the appropriate category, replace judge-specific wrappers with a reusable
interface, and document its assumptions, examples, and original problem.
