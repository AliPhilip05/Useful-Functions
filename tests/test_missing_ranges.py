"""Behavior and boundary checks for the public Python API."""

import contextlib
import io
import itertools
import unittest

from useful_functions.ranges import find_missing_ranges


class MissingRangesTests(unittest.TestCase):
    def test_example(self):
        self.assertEqual(
            find_missing_ranges([0, 1, 3, 50, 75], 0, 99),
            [[2, 2], [4, 49], [51, 74], [76, 99]],
        )

    def test_empty_and_complete_ranges(self):
        self.assertEqual(find_missing_ranges([], -2, 2), [[-2, 2]])
        self.assertEqual(find_missing_ranges([-1], -1, -1), [])
        self.assertEqual(find_missing_ranges([], 0, 0), [[0, 0]])
        self.assertEqual(find_missing_ranges([0, 1, 2], 0, 2), [])

    def test_leading_trailing_and_negative_gaps(self):
        self.assertEqual(find_missing_ranges([-2, 0, 2], -3, 3), [[-3, -3], [-1, -1], [1, 1], [3, 3]])

    def test_accepts_iterators_without_mutating_input(self):
        values = [1, 3]
        self.assertEqual(find_missing_ranges(iter(values), 0, 4), [[0, 0], [2, 2], [4, 4]])
        self.assertEqual(values, [1, 3])

    def test_rejects_invalid_bounds_and_values(self):
        for values, lower, upper in [([], 2, 1), ([2, 1], 0, 3), ([1, 1], 0, 3), ([-1], 0, 3), ([4], 0, 3)]:
            with self.subTest(values=values, lower=lower, upper=upper):
                with self.assertRaises(ValueError):
                    find_missing_ranges(values, lower, upper)

    def test_does_not_print_debug_output(self):
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            find_missing_ranges([1, 3], 0, 4)
        self.assertEqual(output.getvalue(), "")

    def test_ranges_exactly_cover_missing_values(self):
        domain = list(range(-3, 4))
        for size in range(len(domain) + 1):
            for values in itertools.combinations(domain, size):
                result = find_missing_ranges(values, -3, 3)
                expanded = [value for start, end in result for value in range(start, end + 1)]
                self.assertEqual(expanded, sorted(set(domain) - set(values)))
                for previous, current in zip(result, result[1:]):
                    self.assertLess(previous[1] + 1, current[0])


if __name__ == "__main__":
    unittest.main()
