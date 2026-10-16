/*
Given an integer x, return true if x is a palindrome, and false otherwise.

 

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 

Constraints:

-231 <= x <= 231 - 1

*/

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // TODO, declare a vector
        let mut v_is_palindrome = Vec::new();
        let n = x.to_string();
        // TODO, add each element of x into the vector
        for n in n.chars() {
            v_is_palindrome.push(n);
        }
        let mut n = 0;

        // TODO, compare each element of the vector 
        while n < v_is_palindrome.len() {
            if v_is_palindrome[0+n] != v_is_palindrome[v_is_palindrome.len() - n - 1] {
                return false
            } else {
                n = n + 1;
            }
        }
        // TODO, return true otherwise
        true 
    }
}
