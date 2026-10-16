/*202. Happy Number
Solved
Easy

Topics
conpanies icon
Companies
Write an algorithm to determine if a number n is happy.

A happy number is a number defined by the following process:

Starting with any positive integer, replace the number by the sum of the squares of its digits.
Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
Those numbers for which this process ends in 1 are happy.
Return true if n is a happy number, and false if not.

 

Example 1:

Input: n = 19
Output: true
Explanation:
12 + 92 = 82
82 + 22 = 68
62 + 82 = 100
12 + 02 + 02 = 1
Example 2:

Input: n = 2
Output: false
 
 */

fn digit_square_sum(mut n: i32) -> i32 {
    let mut total = 0;
    while n > 0 {
        let d = n % 10;
        total += d * d;
        n /= 10;
    }
    total
}

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = digit_square_sum(slow);
            fast = digit_square_sum(digit_square_sum(fast));

            if fast == 1 {
                return true;
            }
            if slow == fast {
                return false;
            }
        }
    }
}
