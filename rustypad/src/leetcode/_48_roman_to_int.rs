#![allow(unused)]
use std::collections::HashMap;

struct Solution;

impl Solution {

    fn init_values() -> HashMap<char, i32> {
        let hm = [
            ('I', 1), 
            ('V', 5), 
            ('X', 10), 
            ('L', 50), 
            ('C', 100), 
            ('D',  500), 
            ('M', 1000),
        ].iter().cloned().collect::<HashMap<char, i32>>();
        hm
    }
    
    /// ## Roman to Integer
    /// 
    /// https://leetcode.com/problems/roman-to-integer/description/
    ///
    /// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
    /// ```doc
    /// Symbol       Value
    /// I             1
    /// V             5
    /// X             10
    /// L             50
    /// C             100
    /// D             500
    /// M             1000
    /// ```
    /// For example, 2 is written as II in Roman numeral, just two ones added together. 
    /// 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
    ///
    /// Roman numerals are usually written largest to smallest from left to right. However, 
    /// the numeral for four is not IIII. Instead, the number four is written as IV. Because 
    /// the one is before the five we subtract it making four. The same principle applies to 
    /// the number nine, which is written as IX. There are six instances where subtraction is 
    /// used:
    ///
    /// - I can be placed before V (5) and X (10) to make 4 and 9. 
    /// - X can be placed before L (50) and C (100) to make 40 and 90. 
    /// - C can be placed before D (500) and M (1000) to make 400 and 900.
    ///
    /// Given a roman numeral, convert it to an integer.
    ///
    /// Example 1:
    /// ---------
    /// ```
    /// let s = "III".to_string();
    /// assert_eq!(Solution::roman_to_int(s), 3);
    /// ``` 
    /// Explanation: III = 3.
    ///
    /// Example 2:
    /// ---------
    /// ```
    /// let s = "LVIII".to_string();
    /// assert_eq!(Solution::roman_to_int(s), 58);
    /// ``` 
    /// Explanation: L = 50, V= 5, III = 3.
    ///
    /// Example 3:
    /// ---------
    /// ```
    /// let s = "MCMXCIV".to_string();
    /// assert_eq!(Solution::roman_to_int(s), 1994);
    /// ```
    /// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= s.length <= 15
    /// * s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
    /// * It is guaranteed that s is a valid roman numeral in the range [1, 3999].
    /// 
    fn roman_to_int(s: String) -> i32 {
        let values = Solution::init_values();
        
        let mut total = 0;
        let mut i: i32 = 0;
        let characters = s.chars().collect::<Vec<char>>();
        let size = s.len() as i32;
        while i < size {
            // If this is the subtractive case.
            if i + 1 < size && values[&characters[i as usize]] < values[&characters[(i+1) as usize]] {
                let next = values[&characters[(i+1) as usize]];
                let curr = values[&characters[i as usize]];
                total += next - curr;
                i += 2;
            }
            // Else this is NOT the subtractive case.
            else {
                total += values[&characters[i as usize]];
                i += 1;
            }
        }

        total
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_roman_to_int() {
        let s = "MCMXCIV".to_string();
        assert_eq!(Solution::roman_to_int(s), 1994);

        let s = "LVIII".to_string();
        assert_eq!(Solution::roman_to_int(s), 58);

        let s = "III".to_string();
        assert_eq!(Solution::roman_to_int(s), 3);
    }
}