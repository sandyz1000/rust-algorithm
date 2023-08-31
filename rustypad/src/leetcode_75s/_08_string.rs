#![allow(unused)]

use std::collections::HashMap;

/// ## 271. Encode and Decode Strings
/// https://leetcode.com/problems/encode-and-decode-strings/
///
/// Design an algorithm to encode a list of strings to a string. The encoded string is then 
/// sent over the network and is decoded back to the original list of strings.
///
/// Machine 1 (sender) has the function:
///
/// string encode(vector<string> strs) {
///   // ... your code
///   return encoded_string;
/// }
/// Machine 2 (receiver) has the function:
/// vector<string> decode(string s) {
///   //... your code
///   return strs;
/// }
/// So Machine 1 does:
///
/// string encoded_string = encode(strs);
/// and Machine 2 does:
///
/// vector<string> strs2 = decode(encoded_string);
/// strs2 in Machine 2 should be the same as strs in Machine 1.
///
/// Implement the encode and decode methods.
///
/// You are not allowed to solve the problem using any serialize methods (such as eval).
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    // Encode the string to format with prefix of length of string i.e. ["Hello", "World"] can be encoded to "5Hello5World"
    fn new() -> Self {
        Self {  }
    }
	
    fn encode(&self, strs: Vec<String>) -> String {
        let mut ans = String::new();
        for s in strs {
            let length = s.len() as u8;  //  convert to ascii 8-bit
            ans.push(length as char);
            ans.push_str(&s);
        }

        ans
    }
	
    fn decode(&self, s: String) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        let mut c = 0;
        let s: Vec<char> = s.chars().collect();
        while c < s.len() {
            let substr_len = s[c] as u8 as usize;
            c += 1;

            if c + substr_len <= s.len() {
                let slice = &s[c..c+substr_len];
                ans.push(slice.into_iter().collect());
            }
            c += substr_len;
        }

        ans
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */


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
    /// ```
    /// assert_eq(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    /// ```
    /// Explanation: The answer is "abc", with the length of 3.
    ///
    /// Example 2:
    /// ------------
    /// ```
    /// assert_eq(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    /// ```
    /// Explanation: The answer is "b", with the length of 1.
    ///
    /// Example 3:
    /// ------------
    /// ```
    /// assert_eq(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    /// ```
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
        // Once repeating character is found, decrement the count from left until the count is 1
        let mut left = 0;
        let mut ans: i32 = 0;    
        // HashMap to store the character count of each character
        let mut hm: HashMap<char, i32> = HashMap::new();
        let s: Vec<char> = s.chars().collect::<Vec<char>>();

        for right in 0..s.len() {
            let entry = hm.entry(s[right]).or_insert(0);
            *entry += 1;

            while hm[&s[right]] > 1 {
                if let Some(count) = hm.get_mut(&s[left]) {
                    *count -= 1;
                }
                left += 1;
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

    /// ## 20. Valid Parentheses
    ///
    /// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', 
    /// determine if the input string is valid.
    ///
    /// An input string is valid if:
    ///
    /// Open brackets must be closed by the same type of brackets.
    /// Open brackets must be closed in the correct order.
    /// Every close bracket has a corresponding open bracket of the same type.
    ///
    /// Example 1:
    /// ----------  
    /// ```
    /// assert_eq!(Solution::is_valid("()".to_string()), true);
    /// ```
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "()[]{}".to_string();
    /// assert_eq!(Solution::is_valid(s), true);
    /// ```
    /// 
    /// Example 3:
    /// ----------
    /// ```
    /// let s = "(]".to_string();
    /// assert_eq!(Solution::is_valid(s), false);
    /// ```
    /// 
    /// Constraints:
    /// ------------
    /// 1 <= s.length <= 104
    /// s consists of parentheses only '()[]{}'.
    /// 
    pub fn is_valid(s: String) -> bool {
        const BRACKETS: [(char, char); 3] = [('(', ')'), ('[', ']'), ('{', '}')];
        let bracket_map: HashMap<char, char> = BRACKETS.iter().cloned().collect();
        let s: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = vec![];
        for ch in s {
            if bracket_map.contains_key(&ch) {
                stack.push(ch);
            } else {
                // If character starts with closing bracket
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if let Some(curr) = bracket_map.get(&top) {
                    if ch != *curr {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0 
    }

    /// ## 125. Valid Palindrome
    /// https://leetcode.com/problems/valid-palindrome/
    ///
    /// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing 
    /// all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
    ///
    /// Given a string s, return true if it is a palindrome, or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let s = "A man, a plan, a canal: Panama".to_string();
    /// assert_eq!(Solution::is_palindrome(s), true);
    /// ```
    /// Explanation: "amanaplanacanalpanama" is a palindrome.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "race a car".to_string();
    /// assert_eq!(Solution::is_palindrome(s), false);
    /// ```
    /// Explanation: "raceacar" is not a palindrome.
    ///
    /// Example 3:
    /// ------------
    /// ```
    /// let s = " ".to_string();
    /// assert_eq!(Solution::is_palindrome(s), true);
    /// ``` 
    /// Explanation: s is an empty string "" after removing non-alphanumeric characters.
    /// Since an empty string reads the same forward and backward, it is a palindrome.
    ///
    /// Constraints:
    /// ------------
    /// * 1 <= s.length <= 2 * 105
    /// * s consists only of printable ASCII characters.
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars()
            .filter(|x| x.is_alphanumeric()).map(|x| x.to_ascii_lowercase()).collect();
        let mut i = 0;
        while i < s.len() /2 {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
            i += 1;
        } 
        true
    }

    /// ## Longest Palindromic Substring
    /// https://leetcode.com/problems/longest-palindromic-substring/description/
    ///
    /// Given a string s, return the longest palindromic substring in s.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let s = "babad".to_string();
    /// assert_eq!(Solution::longest_palindrome(s), "bab".to_string());
    /// ``` 
    /// Explanation: "aba" is also a valid answer.
    /// 
    /// Example 2:
    /// ---------
    /// ```
    /// let s = "cbbd".to_string();
    /// assert_eq!(Solution::longest_palindrome(s), "bb".to_string()); 
    /// ```
    ///  
    /// Constraints:
    /// -----------
    /// * 1 <= s.length <= 1000
    /// * s consist of only digits and English letters.
    pub fn longest_palindrome(s: String) -> String {
        // TODO: Fix text cases
        fn get_max_length(mut start: i32, mut end: i32, s: &Vec<char>, max_length: &mut i32, max_start: &mut i32) {
            while start > -1 && end < s.len() as i32 && s[start as usize] == s[end as usize] {
                start -= 1;
                end += 1;
            }

            // Return new max_length
            if *max_length < end - start + 1 {
                *max_length = end - start + 1;
                *max_start = start + 1;
            }
        }

        let s: Vec<char> = s.chars().collect();
        let mut max_length = 0;
        let mut start = 0;
        for i in 0..s.len() {
            // For odd case
            get_max_length(i as i32, i as i32, &s, &mut max_length, &mut start);
            // For even case
            get_max_length(i as i32, (i + 1) as i32, &s, &mut max_length, &mut start);
        }

        let ans = s[(start as usize)..(start+max_length) as usize].iter().collect::<String>();
        ans
    }

    /// ## 647. Palindromic Substrings
    ///
    /// Given a string s, return the number of palindromic substrings in it.
    ///
    /// A string is a palindrome when it reads the same backward as forward.
    /// A substring is a contiguous sequence of characters within the string.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let s = "abc".to_string();
    /// assert_eq!(Solution::count_substrings(s), 3);
    /// ```
    /// Explanation: Three palindromic strings: "a", "b", "c".
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "aaa".to_string();
    /// assert_eq!(Solution::count_substrings(s), 6);
    /// ```
    // Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= s.length <= 1000
    /// * s consists of lowercase English letters.
    ///
    pub fn count_substrings(s: String) -> i32 {
        // Expand from center
        let s: Vec<char> = s.chars().collect();
        let count_palindrome = |mut left: i32, mut right: i32| -> i32 {
            let mut count = 0;
            while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
                count += 1;
                left -= 1;
                right += 1;
            }

            count
        };

        (0..s.len()).map(|i| count_palindrome(i as i32, i as i32) + count_palindrome(i as i32, (i + 1) as i32)).sum()
    }

    /// ## Group Anagrams
    /// https://leetcode.com/problems/group-anagrams/
    ///
    /// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
    ///
    /// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
    /// typically using all the original letters exactly once.
    ///
    /// Example 1:
    /// ----------
    /// Input: strs = ["eat","tea","tan","ate","nat","bat"]
    /// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
    ///
    /// Example 2:
    /// ----------
    /// Input: strs = [""]
    /// Output: [[""]]
    ///
    /// Example 3:
    /// ----------
    /// Input: strs = ["a"]
    /// Output: [["a"]]
    ///
    /// Constraints:
    /// ------------
    /// 1 <= strs.length <= 104
    /// 0 <= strs[i].length <= 100
    /// strs[i] consists of lowercase English letters.
    ///
    /// For each word in strs sort the word and insert the word with the sorted key
    /// return values
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = vec![];
        let mut hm: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();
            hm.entry(key).or_insert(vec![]).push(s.clone());
        }

        for (_, value) in hm.iter_mut() {
            value.sort();
            ans.push(value.clone());
        }
        ans
    }

    /// ## 424. Longest Repeating Character Replacement
    /// https://leetcode.com/problems/longest-repeating-character-replacement/description/
    ///
    /// You are given a string s and an integer k. You can choose any character of the string and change it to 
    /// any other uppercase English character. You can perform this operation at most k times.
    ///
    /// Return the length of the longest substring containing the same letter you can get after performing the 
    /// above operations.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let s = "ABAB".to_string(); let k = 2;
    /// assert_eq!(Solution::character_replacement(s, k), 4);
    /// ```
    /// *Explanation*: Replace the two 'A's with two 'B's or vice versa.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "AABABBA".to_string(); let k = 1;
    /// assert_eq!(Solution::character_replacement(s, k), 4);
    /// ```
    /// *Explanation*: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
    /// The substring "BBBB" has the longest repeating letters, which is 4.
    /// There may exists other ways to achive this answer too.
    ///
    /// Constraints:
    /// ------------
    /// * 1 <= s.length <= 105
    /// * s consists of only uppercase English letters.
    /// * 0 <= k <= s.length
    ///
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max_length: i32 = 0;
        let mut max_count = 0;
        let mut left: i32 = 0;
        let s: Vec<char> = s.chars().collect();
        // This will maintain the char count in the current window
        let mut char_dict: Vec<i32> = vec![0; 26];
        for right in 0..s.len() {
            char_dict[s[right] as usize - 'A' as usize] += 1;
            max_count = max_count.max(char_dict[s[right] as usize - 'A' as usize]);
            // If the complement char is > k, move the left pointer  
            while (right as i32) - left - max_count + 1 > k {
                char_dict[s[left as usize] as usize - 'A' as usize] -= 1;
                left += 1;
            }
            max_length = std::cmp::max(max_length, (right as i32) - left + 1);
        }
        max_length
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let codec = Codec::new();
        // Test Case 1
        let dummy_input = vec!["Hello".to_string(), "World".to_string()];
        let ans = codec.encode(dummy_input.clone());
        assert_eq!(codec.decode(ans), dummy_input);

        // Test Case 2
        let dummy_input = vec!["".to_string()];
        let ans = codec.encode(dummy_input.clone());
        assert_eq!(codec.decode(ans), dummy_input);
    }

    #[test]
    fn test_character_replacement() {
        let s = "ABAB".to_string(); let k = 2;
        assert_eq!(Solution::character_replacement(s, k), 4);

        let s = "AABABBA".to_string(); let k = 1;
        assert_eq!(Solution::character_replacement(s, k), 4);
    }

    #[test]
    fn test_group_anagrams() {
        let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let res = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
        assert_eq!(Solution::group_anagrams(strs), res);
    }
    
    #[test]
    fn test_longest_palindrome() {
        let s = "babad".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bab".to_string());
        
        let s = "cbbd".to_string();
        assert_eq!(Solution::longest_palindrome(s), "bb".to_string());
    }

    #[test]
    fn test_count_substrings() {
        let s = "aaa".to_string();
        assert_eq!(Solution::count_substrings(s), 6);

        let s = "abc".to_string();
        assert_eq!(Solution::count_substrings(s), 3);
    }

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring_ii("abcabcbb".to_string()), 3);

        assert_eq!(Solution::length_of_longest_substring_ii("bbbbb".to_string()), 1);

        assert_eq!(Solution::length_of_longest_substring_ii("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_is_valid() {
        let s = "()".to_string();
        let res = Solution::is_valid(s);
        assert_eq!(res, true);

        let s = "()[]{}".to_string();
        let res = Solution::is_valid(s);
        assert_eq!(res, true);

        let s = "(]".to_string();
        let res = Solution::is_valid(s);
        assert_eq!(res, false);
    }

    #[test]
    fn test_is_palindrome() {
        let s = " ".to_string();
        assert_eq!(Solution::is_palindrome(s), true);

        let s = "race a car".to_string();
        assert_eq!(Solution::is_palindrome(s), false);

        let s = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(Solution::is_palindrome(s), true);
    }
}