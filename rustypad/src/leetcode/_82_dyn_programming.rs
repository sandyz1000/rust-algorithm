#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

struct Solution;


impl Solution {
    /// ## 740. Delete and Earn
    /// https://leetcode.com/problems/delete-and-earn/description/
    ///
    /// You are given an integer array nums. You want to maximize the number of points you get by performing 
    /// the following operation any number of times:
    ///
    /// Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal 
    /// to nums[i] - 1 and every element equal to nums[i] + 1.
    /// Return the maximum number of points you can earn by applying the above operation some number of times.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [3,4,2]
    /// 
    /// Output: 6
    /// 
    /// Explanation: You can perform the following operations:
    /// - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
    /// - Delete 2 to earn 2 points. nums = [].
    /// You earn a total of 6 points.
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [2,2,3,3,3,4]
    /// 
    /// Output: 9
    /// 
    /// Explanation: You can perform the following operations:
    /// - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
    /// - Delete a 3 again to earn 3 points. nums = [3].
    /// - Delete a 3 once more to earn 3 points. nums = [].
    /// You earn a total of 9 points.
    ///  
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 2 * 104
    /// 1 <= nums[i] <= 104
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        unimplemented!()    
    }

    /// ## 2140. Solving Questions With Brainpower
    /// https://leetcode.com/problems/solving-questions-with-brainpower/
    ///
    /// You are given a 0-indexed 2D integer array questions where questions[i] = [pointsi, brainpoweri].
    ///
    /// The array describes the questions of an exam, where you have to process the questions in order 
    /// (i.e., starting from question 0) and make a decision whether to solve or skip each question. 
    /// Solving question i will earn you pointsi points but you will be unable to solve each of the next 
    /// brainpoweri questions. If you skip question i, you get to make the decision on the next question.
    ///
    /// For example, given questions = \[\[2, \[3, 2], \[4, 3], \[4, 4], \[2, 5]]:
    /// If question 0 is solved, you will earn 3 points but you will be unable to solve questions 1 and 2.
    /// If instead, question 0 is skipped and question 1 is solved, you will earn 4 points but you will be 
    /// unable to solve questions 2 and 3.
    /// Return the maximum points you can earn for the exam.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let questions = vec![vec![3,2], vec![4,3], vec![4,4], vec![2,5]];
    /// assert_eq!(Solution::most_points(questions), 5);
    /// ```
    /// Explanation: The maximum points can be earned by solving questions 0 and 3.
    /// - Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
    /// - Unable to solve questions 1 and 2
    /// - Solve question 3: Earn 2 points
    /// Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
    /// 
    /// Example 2:
    /// -----------
    /// ```
    /// let questions = vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]];
    /// assert_eq!(Solution::most_points(questions), 7);
    /// ```
    /// Explanation: The maximum points can be earned by solving questions 1 and 4.
    /// - Skip question 0
    /// - Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
    /// - Unable to solve questions 2 and 3
    /// - Solve question 4: Earn 5 points
    /// Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
    ///
    /// Constraints:
    /// ----------- 
    /// - 1 <= questions.length <= 105
    /// - questions\[i].length == 2
    /// - 1 <= pointsi, brainpoweri <= 105
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        fn get_max_points(
            idx: i32, 
            questions: &Vec<Vec<i32>>,
            cache: &mut HashMap<i32, i64>
        ) -> i64 {
            // Base case
            if idx >= questions.len() as i32 {
                return 0;
            }
            if cache.contains_key(&idx) {
                return cache[&idx];
            }
            // Score at current index
            let mut score = questions[idx as usize][0] as i64 + 
                get_max_points(idx + questions[idx as usize][1] + 1, questions, cache);
            
            // Skip the current index
            score = score.max(get_max_points(idx+1, questions, cache));
            cache.insert(idx, score);
            cache[&idx]
        } 

        let mut cache: HashMap<i32, i64> = HashMap::new();
        get_max_points(0, &questions, &mut cache)
    }

    /// ## 1143. Longest Common Subsequence
    /// https://leetcode.com/problems/longest-common-subsequence/
    ///
    /// Given two strings text1 and text2, return the length of their longest common subsequence. 
    /// If there is no common subsequence, return 0.
    ///
    /// A subsequence of a string is a new string generated from the original string with some 
    /// characters (can be none) deleted without changing the relative order of the remaining characters.
    ///
    /// For example, "ace" is a subsequence of "abcde".
    /// A common subsequence of two strings is a subsequence that is common to both strings.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let text1 = "abcde".to_string(); let text2 = "ace".to_string(); 
    /// assert_eq(Solution::longest_common_subsequence(text1, text2), 3);
    /// ```
    /// Explanation: The longest common subsequence is "ace" and its length is 3.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let text1 = "abc".to_string(); let text2 = "abc".to_string();
    /// assert_eq(Solution::longest_common_subsequence(text1, text2), 3);
    /// ```
    /// Explanation: The longest common subsequence is "abc" and its length is 3.
    /// Example 3:
    /// ---------
    /// ```
    /// let text1 = "abc".to_string(); let text2 = "def".to_string();
    /// assert_eq(Solution::longest_common_subsequence(text1, text2), 0);
    /// ```
    /// Explanation: There is no such common subsequence, so the result is 0.
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= text1.length, text2.length <= 1000
    /// * text1 and text2 consist of only lowercase English characters.
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        fn memoizer_fn<K1, K2, V, F>(func: F) -> impl FnMut(K1, K2, &Vec<char>, &Vec<char>) -> V
            where F: Fn(K1, K2, &Vec<char>, &Vec<char>) -> V,
                K1: Copy,
                K2: Copy,
                V: Copy,
                (K1, K2): Hash + std::cmp::Eq + PartialEq + Clone,
        {
            let mut cache: HashMap<(K1, K2), V> = HashMap::new();

            move |arg1: K1, arg2: K2, text1: &Vec<char>, text2: &Vec<char>| {
                if let Some(result) = cache.get(&(arg1, arg2)) {
                    *result
                } else {
                    let result = func(arg1, arg2, text1, text2);
                    cache.insert((arg1, arg2), result);
                    result
                }
            }
        }
        
        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();
        
        let mut func = memoizer_fn::<usize, usize, i32, _>(find_lcs);

        fn find_lcs(
            pos1: usize, pos2: usize, 
            text1: &Vec<char>, text2: &Vec<char>
        ) -> i32 {
        
            // Base case
            if pos1 >= text1.len() || pos2 >= text2.len() {
                return 0
            }

            let mut ans = 0;
            
            if text1[pos1] == text2[pos2] {
                ans = 1 + memoizer_fn(find_lcs)(pos1+1, pos2+1, text1, text2);
                return ans;
            }

            ans = find_lcs(pos1+1, pos2, text1, text2)
                .max(find_lcs(pos1, pos2+1, text1, text2));
            
            ans
        }
        
        func(0, 0, &text1, &text2)

    }

    /// ## 1143. Longest Common Subsequence
    /// 
    /// Dynamic Programming Solution for Longest Common Subsequence
    /// ```
    /// let s1 = "ACADB";
    /// let s2 = "CBDA";
    /// let m = s1.len();
    /// let n = s2.len();
    /// lcs_algo(S1, S2, m, n);
    /// ```
    fn lcs_dp(s1: &str, s2: &str, m: usize, n: usize) -> String {
        let mut lcs = vec![vec![0; n + 1]; m + 1];
    
        // Building the matrix in bottom-up way
        for i in 0..=m {
            for j in 0..=n {
                if i == 0 || j == 0 {
                    lcs[i][j] = 0;
                } else if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                    lcs[i][j] = lcs[i - 1][j - 1] + 1;
                } else {
                    lcs[i][j] = std::cmp::max(lcs[i - 1][j], lcs[i][j - 1]);
                }
            }
        }
    
        let mut index: i32 = lcs[m][n];
    
        let mut lcs_algo = vec![""; (index + 1) as usize];
        lcs_algo[index as usize] = "";
    
        let mut i = m;
        let mut j = n;
        while i > 0 && j > 0 {
            if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
                lcs_algo[(index - 1) as usize] = &s1[(i - 1)..i];
                i -= 1;
                j -= 1;
                index -= 1;
            } else if lcs[i - 1][j] > lcs[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
    
        // Printing the sub sequences
        lcs_algo.join("")
    }
    
    /// ## 2218. Maximum Value of K Coins From Piles
    /// https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/
    ///
    /// There are n piles of coins on a table. Each pile consists of a positive number of coins 
    /// of assorted denominations.
    ///
    /// In one move, you can choose any coin on top of any pile, remove it, and add it to your wallet.
    ///
    /// Given a list piles, where piles[i] is a list of integers denoting the composition of the ith 
    /// pile from top to bottom, and a positive integer k, return the maximum total value of coins you 
    /// can have in your wallet if you choose exactly k coins optimally.
    ///
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let piles = vec![vec![1,100,3],vec![7,8,9]]; let k = 2;
    /// assert_eq!(Solution::max_value_of_coins(piles, k), 101);
    /// ```
    /// Explanation:
    /// The above diagram shows the different ways we can choose k coins.
    /// The maximum total we can obtain is 101.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let piles = vec![
    /// vec![100],vec![100],vec![100],vec![100],vec![100],vec![100],
    /// vec![1,1,1,1,1,1,700]
    /// ]; 
    /// let k = 7;
    /// assert_eq!(Solution::max_value_of_coins(piles, k), 706);
    /// ```
    /// Explanation:
    /// The maximum total can be obtained if we choose all coins from the last pile.
    ///
    /// Constraints:
    /// --------------
    /// * n == piles.length
    /// * 1 <= n <= 1000
    /// * 1 <= piles[i][j] <= 105
    /// * 1 <= k <= sum(piles[i].length) <= 2000
    ///
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        unimplemented!()
    }


    pub fn num_decodings(s: String) -> i32 {
        fn recursive_fn(index: i32, s: &Vec<char>, cache: &mut HashMap<i32, i32>) -> i32 {
            if cache.contains_key(&index) {
                return cache[&index];
            }
            // if index reach the end of string
            if index == s.len() as i32{
                return 1;
            }
            
            if s[index as usize] == '0' {
                return 0;
            }
            
            if index == s.len() as i32 - 1{
                return 1;
            }

            let mut ans = recursive_fn(index + 1, s, cache);
            let digit = &s[(index as usize)..(index as usize)+2].to_vec().iter().collect::<String>();
            let digit: i32 = digit.parse().expect("Cannot parse digit");
            if  digit <= 26 {
                ans += recursive_fn(index + 2, s, cache);
            }
            
            cache.insert(index, ans);
            ans
        }

        let s = s.chars().collect();
        let mut cache: HashMap<i32, i32> = HashMap::new();
        recursive_fn(0, &s, &mut cache)
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_points(){
        let questions = vec![vec![1,1],vec![2,2],vec![3,3],vec![4,4],vec![5,5]];
        assert_eq!(Solution::most_points(questions), 7);

        let questions = vec![vec![3,2], vec![4,3], vec![4,4], vec![2,5]];
        assert_eq!(Solution::most_points(questions), 5);
    }
    
    #[test]
    fn test_delete_and_earn() {
        let nums = vec![3,4,2];
        assert_eq!(Solution::delete_and_earn(nums), 6);

        let nums = vec![2,2,3,3,3,4];
        assert_eq!(Solution::delete_and_earn(nums), 9);
    }

    #[test]
    fn test_longest_common_subsequence() {
        // let text1 = "abcde".to_string(); let text2 = "ace".to_string();
        // assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        // let text1 = "abc".to_string(); let text2 = "abc".to_string();
        // assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);

        // let text1 = "abc".to_string(); let text2 = "def".to_string();
        // assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);

        let text1 = "pmjghexybyrgzczy".to_string(); let text2 = "hafcdqbgncrcbihkd".to_string();
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 14);

        let s1 = "ACADB"; let s2 = "CBDA";
        let m = s1.len(); let n = s2.len();
        let res = Solution::lcs_dp(s1, s2, m, n);
    }

}