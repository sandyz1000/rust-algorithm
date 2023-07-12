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
        unimplemented!()
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