#![allow(unused)]

use std::collections::HashMap;

struct Solution;


impl Solution {
    /// ## 12. Integer to Roman
    /// https://leetcode.com/problems/integer-to-roman/description/
    ///
    /// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
    ///
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
    ///
    /// For example, 2 is written as II in Roman numeral, just two one's added together. 
    /// 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, 
    /// which is XX + V + II.
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
    ///
    /// Given an integer, convert it to a roman numeral. 
    ///
    /// Example 1:
    /// ---------
    /// ```
    /// let num = 3;
    /// assert_eq!(Solution::int_to_roman(num), "III".to_string());
    /// ```
    /// Explanation: 3 is represented as 3 ones.
    ///
    /// Example 2:
    /// ---------
    /// ```
    /// let num = 58;
    /// assert_eq!(Solution::int_to_roman(num), "LVIII".to_string());
    /// ```
    /// Explanation: L = 50, V = 5, III = 3.
    ///
    /// Example 3:
    /// ---------
    /// ```
    /// let num = 1994;
    /// assert_eq!(Solution::int_to_roman(num), "MCMXCIV".to_string());
    /// ```
    /// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= num <= 3999
    ///
    pub fn int_to_roman(mut num: i32) -> String {
        let mut values = vec![
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), 
            (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), 
            (5, "V"), (4, "IV"), (1, "I")
        ];
        
        let mut ans: String = String::new();
        for (v, c) in values.into_iter() {
            if num == 0 {
                break;
            }
            let mut quotient = num / v;
            num = num % v; 
            let mut val = String::new();
            while quotient > 0 {
                val.push_str(c);
                quotient -= 1;
            }
            ans.push_str(val.as_str());
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num = 3;
        assert_eq!(Solution::int_to_roman(num), "III".to_string());

        let num = 58;
        assert_eq!(Solution::int_to_roman(num), "LVIII".to_string());

        let num = 1994;
        assert_eq!(Solution::int_to_roman(num), "MCMXCIV".to_string());
    }
}