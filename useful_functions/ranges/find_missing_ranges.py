"""Find gaps in a sorted sequence of integers (adapted from LeetCode 163)."""

from typing import Iterable


def find_missing_ranges(
    values: Iterable[int], lower: int, upper: int
) -> list[list[int]]:
    """Return inclusive ranges covering every missing integer in [lower, upper].

    Values must be strictly increasing, unique, and within the requested bounds.
    Invalid bounds or values raise ValueError. An empty iterable returns the
    entire requested range. Runs in O(n) time plus the space for the result.

    >>> find_missing_ranges([0, 1, 3, 50, 75], 0, 99)
    [[2, 2], [4, 49], [51, 74], [76, 99]]
    """
    if lower > upper:
        raise ValueError("lower must be less than or equal to upper")

    missing = []
    next_missing = lower
    previous = None
    for value in values:
        if value < lower or value > upper:
            raise ValueError("values must be within [lower, upper]")
        if previous is not None and value <= previous:
            raise ValueError("values must be strictly increasing and unique")
        if next_missing < value:
            missing.append([next_missing, value - 1])
        next_missing = value + 1
        previous = value

    if next_missing <= upper:
        missing.append([next_missing, upper])
    return missing
