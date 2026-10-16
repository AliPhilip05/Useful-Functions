impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        //Go through the list
        let mut reversed_chars: Vec<char> = Vec::new();
        let mut index_chars: Vec<usize> = Vec::new();
        let mut index: usize = 0;
        let original: Vec<char> = s.chars().collect();

        for chars in s.chars() {
            if chars == 'a' || chars == 'e' || chars == 'i' || chars == 'o' || chars == 'u'
                || chars == 'A' || chars == 'E' || chars == 'I' || chars == 'O' || chars == 'U' {
                index_chars.push(index)
            }
            index += 1;
        }

        // reverse that list or vector
        index_chars.reverse();

        // push into a vector a list of the chars that are vowels
        index = 0;
        for chars in s.chars() {
            if chars == 'a' || chars == 'e' || chars == 'i' || chars == 'o' || chars == 'u'
                || chars == 'A' || chars == 'E' || chars == 'I' || chars == 'O' || chars == 'U' {
                // push the vowel from the mirrored position, not the current one
                let temp_index = index_chars[index];
                reversed_chars.push(original[temp_index]);
                index += 1;
            } else {
                reversed_chars.push(chars);
            }
        }

        // construct a new string using that vector and the original list
        let mut string: String = String::from("");
        for chars in reversed_chars {
            string += &chars.to_string();
        }
        string
    }
}
