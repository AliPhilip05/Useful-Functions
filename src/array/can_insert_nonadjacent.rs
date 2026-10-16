/*
605. Can Place Flowers
Solved
Easy

Topics
conpanies icon
Companies
You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

 

Example 1:

Input: flowerbed = [1,0,0,0,1], n = 1
Output: true
Example 2:

Input: flowerbed = [1,0,0,0,1], n = 2
Output: false
 

Constraints:

1 <= flowerbed.length <= 2 * 104
flowerbed[i] is 0 or 1.
There are no two adjacent flowers in flowerbed.
0 <= n <= flowerbed.length
*/

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
            let size_of_flowerbed:usize = flowerbed.len() - 1;
            let mut can_place_flower:i32 = 0;
            let mut previous_flower:i32 = 0;
            for i in 0..size_of_flowerbed {
                    let current_flower:i32 = flowerbed[i];
                    let next_flower:i32 = flowerbed[i+1];
                    if previous_flower == 0 && current_flower == 0 && next_flower == 0 {
                                previous_flower = 1;
                                can_place_flower += 1;
                    }
                    else {
                                previous_flower = current_flower;
                    }
            }
            if flowerbed[size_of_flowerbed] == 0 && previous_flower == 0 {
                    can_place_flower += 1;
            }
            can_place_flower >= n
    }
}
