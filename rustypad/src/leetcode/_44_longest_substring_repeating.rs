#![allow(unused)]

use std::collections::HashMap;


struct Solution;

impl Solution {

    /// ## 3. Longest Substring Without Repeating Characters
    /// https://leetcode.com/problems/longest-substring-without-repeating-characters/
    ///
    ///
    /// Given a string s, find the length of the longest  substring without repeating characters.
    ///
    /// Example 1:
    /// ------------
    /// Input: s = "abcabcbb"
    /// Output: 3
    /// Explanation: The answer is "abc", with the length of 3.
    ///
    /// Example 2:
    /// ------------
    /// Input: s = "bbbbb"
    /// Output: 1
    /// Explanation: The answer is "b", with the length of 1.
    ///
    /// Example 3:
    /// ------------
    /// Input: s = "pwwkew"
    /// Output: 3
    /// Explanation: The answer is "wke", with the length of 3.
    /// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
    ///
    ///
    /// Constraints:
    /// ------------
    /// 0 <= s.length <= 5 * 104
    /// s consists of English letters, digits, symbols and spaces.
    /// 
    pub fn length_of_longest_substring_ii(s: String) -> i32 {
        // Two pointer left and right that increment by 1 and save the character count in hashmap
        // 
        let mut left = 0;
        let mut ans: i32 = 0;    
        // HashMap to store the character count of each character
        let mut hm: HashMap<char, i32> = HashMap::new();
        let s: Vec<char> = s.chars().collect::<Vec<char>>();

        for right in 0..s.len() {
            let entry = hm.entry(s[right]).or_insert(0);
            *entry += 1;

            while let Some(v) = hm.get_mut(&s[left]) {
                if *v > 1 {
                    *v -= 1;
                    left += 1;
                } else {
                    break;
                }
            }
            
            ans = ans.max((right - left + 1) as i32);
        }

        ans
    
    }

    fn length_of_longest_substring(s: String) -> i32 {
        // Two pointer with right pointer move to right and check for duplicate char
        // Set for maintaining duplicate character count
        // Max-length
        let mut seen = [0; 128];
        let (mut left, mut right) = (0, 0);
        let mut max_length = 0;
        for c in s.chars() {
            let current_idx = c as usize;
            seen[current_idx] += 1;
            while seen[current_idx] > 1 {
                seen[s.chars().nth(left).unwrap() as usize] -= 1;
                left += 1;
            }
            max_length = max_length.max(right - left + 1);
            right += 1;
        }
        max_length as i32
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);

        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);

        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}