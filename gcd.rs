/*
1071. Greatest Common Divisor of Strings
Solved
Easy

Topics
conpanies icon
Companies

Hint
For two strings s and t, we say "t divides s" if and only if s = t + t + t + ... + t + t (i.e., t is concatenated with itself one or more times).

Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

 

Example 1:

Input: str1 = "ABCABC", str2 = "ABC"

Output: "ABC"

Example 2:

Input: str1 = "ABABAB", str2 = "ABAB"

Output: "AB"

Example 3:

Input: str1 = "LEET", str2 = "CODE"

Output: ""

Example 4:

Input: str1 = "AAAAAB", str2 = "AAA"

Output: ""​​​​​​​

 

Constraints:

1 <= str1.length, str2.length <= 1000
str1 and str2 consist of English uppercase letters.
*/
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
            let mut gc_string:String = String::new();
            let mut gcd_number:usize = str2.len();
            while gcd_number > 0 {

                    if str2.len() % gcd_number == 0 
                    && str1.len() % gcd_number == 0 {
                            
                            let str1_multiple:usize = str1.len() / gcd_number;
                            let str2_multiple:usize = str2.len() / gcd_number;
                            let temp_str:String = str2[0..gcd_number].to_string();

                            if &temp_str.repeat(str1_multiple) == &str1 
                            && &temp_str.repeat(str2_multiple) == &str2 {
                                    return temp_str;
                            }
                     }
                    gcd_number -= 1;
            }

            return gc_string;
    }
}
