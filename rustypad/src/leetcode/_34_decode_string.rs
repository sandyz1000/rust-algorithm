#![allow(unused)]

#[derive(Debug)]
struct Solution;

impl Solution {
    /// ## 394. Decode String
    /// https://leetcode.com/problems/decode-string/description/
    ///
    /// Given an encoded string, return its decoded string.
    ///
    /// The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being 
    /// repeated exactly k times. Note that k is guaranteed to be a positive integer.
    ///
    /// You may assume that the input string is always valid; there are no extra white spaces, square brackets 
    /// are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits 
    /// and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].
    ///
    /// The test cases are generated so that the length of the output will never exceed 105.
    ///
    /// Example 1:
    /// ---------
    /// ```
    /// let s = "3[a]2[bc]".to_string();
    /// assert_eq!(Solution::decode_string(s), "aaabcbc".to_string());
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "3[a2[c]]".to_string(); 
    /// assert_eq!(Solution::decode_string(s), "accaccacc".to_string());
    /// ```
    /// 
    /// Example 3:
    /// ----------
    /// ```
    /// let s = "2[abc]3[cd]ef".to_string();
    /// assert_eq!(Solution::decode_string(s), "abcabccdcdcdef".to_string());
    /// ```
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= s.length <= 30
    /// - s consists of lowercase English letters, digits, and square brackets '[]'.
    /// - s is guaranteed to be a valid input.
    /// - All the integers in s are in the range [1, 300].
    pub fn decode_string(s: String) -> String {
        // Two stack one for int and other for character
        // Parse digit and create valid integer
        // Look for opening bracket and add into stack
        // If closing bracket; then pop the int from stack and pop every chacter until [
        // Build the string and insert to character arr

        let mut int_stack: Vec<i32> = vec![];
        let mut char_stack: Vec<char> = vec![];
        let s: Vec<char> = s.chars().into_iter().collect();
        let mut c = 0;
        while c < s.len() {
            let mut curr_digit = 0;
            while s[c].is_digit(10) {
                curr_digit = curr_digit * 10 + s[c].to_digit(10).unwrap() as i32;
                c+= 1;
            }
            if curr_digit != 0 {
                int_stack.push(curr_digit);
                continue;
            }

            if s[c] == ']' {
                // Pop all the character from the character stack
                let mut ans = Vec::<char>::new();
                while !char_stack.is_empty() {
                    if *char_stack.last().unwrap() == '[' {
                        char_stack.pop();
                        break;
                    }
                    if char_stack.last().unwrap().is_alphabetic() {
                        ans.push(char_stack.pop().unwrap());
                    }
                }
                ans.reverse();
                let ans: String = ans.iter().collect();
                // Pop the int from the int stack
                let ans_int = int_stack.pop().unwrap();
                let mut c1 = String::new();
                for i in 0..ans_int {
                    c1.push_str(ans.as_str());
                }
                char_stack.extend(c1.chars());

            } else {
                char_stack.push(s[c]);
            }

            c+= 1;
        }
        
        char_stack.iter().collect()

    }
}

mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let s = "3[a]2[bc]".to_string();
        assert_eq!(Solution::decode_string(s), "aaabcbc".to_string());

        let s = "3[a2[c]]".to_string();
        assert_eq!(Solution::decode_string(s), "accaccacc".to_string());

        let s = "2[abc]3[cd]ef".to_string();
        assert_eq!(Solution::decode_string(s), "abcabccdcdcdef".to_string());
    }
}