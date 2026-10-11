/*
1768. Merge Strings Alternately
Solved
Easy

Topics
conpanies icon
Companies

Hint
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.

 

Example 1:

Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r
Example 2:

Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b 
word2:    p   q   r   s
merged: a p b q   r   s
Example 3:

Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q 
merged: a p b q c   d
 

Constraints:

1 <= word1.length, word2.length <= 100
word1 and word2 consist of lowercase English letters.


*/

impl Solution {
    pub fn merge_string(word1:&String, word2:&String) -> String {
        let mut merged_string: String = String::new();
        for (chars,chars2) in word1.chars().zip(word2.chars()) {
            merged_string += &chars.to_string();
            merged_string += &chars2.to_string();
        }
        merged_string
    }

    pub fn append_string(word:String, longer_word:String) -> String {
        let len:usize = word.len() / 2;
        let appended_string:String = word + &longer_word[len..];
        appended_string
    }

    pub fn merge_alternately(word1: String, word2: String) -> String {

        if word1.len() > word2.len() {
            let answer:String = Self::merge_string(&word1,&word2);
            return Self::append_string(answer,word1);
        }
        else if word1.len() < word2.len() {
            let answer:String = Self::merge_string(&word1,&word2);
            return Self::append_string(answer,word2);
        } 
        else {
            let answer:String = Self::merge_string(&word1,&word2);
            return answer;
        }
        
    }
}
