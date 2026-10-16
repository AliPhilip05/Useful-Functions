/* 
151. Reverse Words in a String
Solved
Medium

Topics
conpanies icon
Companies
Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

Return a string of the words in reverse order concatenated by a single space.

Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

 

Example 1:

Input: s = "the sky is blue"
Output: "blue is sky the"
Example 2:

Input: s = "  hello world  "
Output: "world hello"
Explanation: Your reversed string should not contain leading or trailing spaces.
Example 3:

Input: s = "a good   example"
Output: "example good a"
Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
*/
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut start:usize = 0;
        let mut end:usize = 0;
        let mut vector: Vec<&str> = Vec::new();
        for character in s.chars() {
            if character.to_string() != " " {
                end += 1;
            }
            else if character.to_string() == " " && start < end{
                
                vector.push(&s[start..end]);
                end += 1;
                start = end;
            }
            else {
                end += 1;
                start = end;
            }
        } 
        vector.push(&s[start..end]);
        for elements in &vector {
        }
        let mut reversed:String = String::new();
        vector.reverse();
        for element in vector {
            reversed.push_str(element);
            reversed.push_str(" ");
        }

        reversed.trim().to_string()
    }
}
