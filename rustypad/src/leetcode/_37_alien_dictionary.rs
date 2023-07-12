#![allow(unused)]

struct Solution;

impl Solution {
    /// ## Alien Dictionary (Hard)
    /// https://leetcode.com/problems/alien-dictionary/description/
    ///
    /// There is a new alien language that uses the English alphabet. However, the order 
    /// among the letters is unknown to you.
    ///
    /// You are given a list of strings words from the alien language's dictionary, where 
    /// the strings in words are  sorted lexicographically by the rules of this new language.
    ///
    /// Return a string of the unique letters in the new alien language sorted in 
    /// lexicographically increasing order by the new language's rules. If there is no solution, 
    /// return "". 
    /// If there are multiple solutions, return any of them.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let words = vec!["wrt","wrf","er","ett","rftt"].map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::alien_order(words), "wertf".to_string());
    /// ```
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let words = vec!["z","x"].map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::alien_order(words), "zx".to_string());
    /// ```
    /// 
    /// Example 3:
    /// ----------
    /// ```
    /// let words = vec!["z","x","z"].map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::alien_order(words), "".to_string());
    /// ```
    /// Explanation: The order is invalid, so return "".
    ///
    /// Constraints:
    /// ------------
    /// 1 <= words.length <= 100
    /// 1 <= words[i].length <= 100
    /// words[i] consists of only lowercase English letters.
    pub fn alien_order(words: Vec<String>) -> String {
        unimplemented!()
    }
}   


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let words = vec!["z","x"].iter().map(|&x| x.to_string()).collect();
        assert_eq!(Solution::alien_order(words), "zx".to_string());

        let words = vec!["z","x","z"].iter().map(|&x| x.to_string()).collect();
        assert_eq!(Solution::alien_order(words), "".to_string());

        let words = vec!["wrt","wrf","er","ett","rftt"].iter().map(|&x| x.to_string()).collect();
        assert_eq!(Solution::alien_order(words), "wertf".to_string());
    }
}