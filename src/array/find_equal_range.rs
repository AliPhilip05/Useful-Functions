/*
34. Find First and Last Position of Element in Sorted Array
Solved
Medium

Topics
conpanies icon
Companies
Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

If target is not found in the array, return [-1, -1].

You must write an algorithm with O(log n) runtime complexity.

 

Example 1:

Input: nums = [5,7,7,8,8,10], target = 8
Output: [3,4]
Example 2:

Input: nums = [5,7,7,8,8,10], target = 6
Output: [-1,-1]
Example 3:

Input: nums = [], target = 0
Output: [-1,-1]
 

Constraints:

0 <= nums.length <= 105
-109 <= nums[i] <= 109
nums is a non-decreasing array.
-109 <= target <= 109

*/

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut position = Vec::new();
        let length: usize = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = length as i32;
        let mut iterator: usize = length / 2;
        let mut found = false;

        while left != right {
            if nums[iterator] > target {
                right = iterator as i32;
            } else if nums[iterator] < target {
                left = iterator as i32 + 1;
            } else {
                found = true;
                break;
            }
            iterator = ((left + right) / 2) as usize;
        }

        if nums.is_empty() || !found {
            return vec![-1, -1];
        }

        // Binary search for the leftmost occurrence of target
        let mut lo: i32 = 0;
        let mut hi: i32 = iterator as i32;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        let final_left = lo;

        // Binary search for the rightmost occurrence of target
        let mut lo: i32 = iterator as i32;
        let mut hi: i32 = length as i32 - 1;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if nums[mid as usize] > target {
                hi = mid - 1;
            } else {
                lo = mid;
            }
        }
        let final_right = lo;

        position.push(final_left);
        position.push(final_right);
        position
    }
}
