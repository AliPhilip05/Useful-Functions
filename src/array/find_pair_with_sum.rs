/*
You are given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

 

Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]
Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]
*/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
       let mut i: usize = 0;

       //incriment througout the entire vector 
       while i < nums.len() {
        let mut j: usize = i + 1;
         
          //incriment througout the vector past the current integer  
        while j < nums.len() {
            let two_sum = nums[i] + nums[j];
            if two_sum == target {
                return vec![i as i32,j as i32];
            }
             j += 1;
        }

        i += 1;
       }
       return vec![];
    }
}
