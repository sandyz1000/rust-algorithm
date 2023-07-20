#![allow(unused)]

use std::hash::Hash;
use std::collections::{HashSet, HashMap};

struct Solution;

impl Solution {
    /// ##  Running Sum of 1d Array
    /// Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
    ///
    /// Return the running sum of nums.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [1,2,3,4]
    /// Output: [1,3,6,10]
    /// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [1,1,1,1,1]
    /// Output: [1,2,3,4,5]
    /// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
    /// 
    /// Example 3:
    /// ----------
    /// Input: nums = [3,1,2,10,1]
    /// Output: [3,4,6,16,17]
    

    /// Constraints:

    /// 1 <= nums.length <= 1000
    /// -10^6 <= nums[i] <= 10^6
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        for i in 0..nums.len() {
            ans.push(nums[i] + ans.last().unwrap_or(&0));
        }

        ans
    }

    /// Given an array of integers nums, you start with an initial positive value startValue.
    ///
    /// In each iteration, you calculate the step by step sum of startValue plus elements in nums 
    /// (from left to right).
    ///
    /// Return the minimum positive value of startValue such that the step by step sum is never less 
    /// than 1.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [-3,2,-3,4,2]
    /// Output: 5
    /// Explanation: If you choose startValue = 4, in the third iteration your step by step sum is less 
    /// than 1.
    /// 
    /// step by step sum
    /// startValue = 4 | startValue = 5 | nums
    /// (4 -3 ) = 1  | (5 -3 ) = 2    |  -3
    /// (1 +2 ) = 3  | (2 +2 ) = 4    |   2
    /// (3 -3 ) = 0  | (4 -3 ) = 1    |  -3
    /// (0 +4 ) = 4  | (1 +4 ) = 5    |   4
    /// (4 +2 ) = 6  | (5 +2 ) = 7    |   2
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [1,2]
    /// Output: 1
    /// Explanation: Minimum start value should be positive. 
    ///
    /// Example 3:
    /// ----------
    /// Input: nums = [1,-2,-3] -> [1, -1, -4]
    /// Output: 5
    ///
    /// Constraints:
    /// ----------
    /// 1 <= nums.length <= 100
    /// -100 <= nums[i] <= 100
    /// 
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let prefix_sum: Vec<i32> = Solution::running_sum(nums);  // [-3, -1, -4, 0, 2]
        let min_value: &i32 = prefix_sum.iter().min().unwrap();  // -3
        if *min_value >= 0 {
            1
        } else {
            (1 - *min_value).abs()
        }
    }


    /// ## K Radius Subarray Averages
    ///
    /// You are given a 0-indexed array nums of n integers, and an integer k.
    ///
    /// The k-radius average for a subarray of nums centered at some index i with the radius k is 
    /// the average of all elements in nums between the indices i - k and i + k (inclusive). If 
    /// there are less than k elements before or after the index i, then the k-radius average is -1.
    ///
    /// Build and return an array avgs of length n where avgs[i] is the k-radius average for the 
    /// subarray centered at index i.
    ///
    /// The average of x elements is the sum of the x elements divided by x, using integer division. 
    /// The integer division truncates toward zero, which means losing its fractional part.
    ///
    /// For example, the average of four elements 2, 3, 1, and 5 is (2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75, 
    /// which truncates to 2.
    ///
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [7,4,3,9,1,8,5,2,6], k = 3
    /// Output: [-1,-1,-1,5,4,4,-1,-1,-1]
    /// Explanation:
    /// - avg[0], avg[1], and avg[2] are -1 because there are less than k elements before each index.
    /// - The sum of the subarray centered at index 3 with radius 3 is: 7 + 4 + 3 + 9 + 1 + 8 + 5 = 37.
    /// Using integer division, avg[3] = 37 / 7 = 5.
    /// - For the subarray centered at index 4, avg[4] = (4 + 3 + 9 + 1 + 8 + 5 + 2) / 7 = 4.
    /// - For the subarray centered at index 5, avg[5] = (3 + 9 + 1 + 8 + 5 + 2 + 6) / 7 = 4.
    /// - avg[6], avg[7], and avg[8] are -1 because there are less than k elements after each index.
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [100000], k = 0
    /// Output: [100000]
    /// Explanation:
    /// - The sum of the subarray centered at index 0 with radius 0 is: 100000.
    /// avg[0] = 100000 / 1 = 100000.
    /// 
    /// Example 3:
    /// ----------
    /// Input: nums = [8], k = 100000
    /// Output: [-1]
    /// Explanation: 
    /// - avg[0] is -1 because there are less than k elements before and after index 0.
    ///
    /// Constraints:
    /// ----------
    /// n == nums.length
    /// 1 <= n <= 105
    /// 0 <= nums[i], k <= 105
    /// 
    pub fn get_averages_ii(nums: Vec<i32>, k: i32) -> Vec<i32> {
        fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
            let mut ans: Vec<i32> = vec![];
            for i in 0..nums.len() {
                ans.push(ans.last().unwrap_or(&0) + nums[i]);
            }
            ans
        }

        let mut ans: Vec<i32> = vec![-1; nums.len()];
        let prefix: Vec<i32> = running_sum(&nums);
        let k: usize = k as usize;

        for i in 0..nums.len() {
            if i >= k && i + k < nums.len() {
                let value = (prefix[i + k] - prefix[i - k]) / (2*k + 1) as i32;
                ans.push(value);
            }
        } 

        ans
    }

    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut ans: Vec<i32> = vec![-1; nums.len()];
        let mut prefix_sum = 0;
        let mut start = 0;
        for (end, num) in nums.iter().enumerate() {
            prefix_sum += num;
            if (end as i32) >= 2 * k {
                ans[(start + k) as usize] = prefix_sum / (2 * k + 1);
                prefix_sum -= nums[start as usize];
                start += 1;
            }
        }
        ans
    }

    /// A pangram is a sentence where every letter of the English alphabet appears at least once.
    ///
    /// Given a string sentence containing only lowercase English letters, return true if sentence 
    /// is a pangram, or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// Input: sentence = "thequickbrownfoxjumpsoverthelazydog"
    /// Output: true
    /// Explanation: sentence contains at least one of every letter of the English alphabet.
    ///
    /// Example 2:
    /// ----------
    /// Input: sentence = "leetcode"
    /// Output: false
    ///
    /// Constraints:
    /// ------------
    /// 1 <= sentence.length <= 1000
    /// sentence consists of lowercase English letters.
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut char_set: HashSet<char> = HashSet::new();
        for c in sentence.chars() {
            if !char_set.contains(&c) {
                char_set.insert(c.clone());
            }
        }
        
        char_set.len() == 26
    }

    /// https://leetcode.com/problems/missing-number/description/ 
    /// Given an array nums containing n distinct numbers in the range [0, n], return the only number 
    /// in the range that is missing from the array.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [3,0,1]
    /// Output: 2
    /// 
    /// Explanation: n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 
    /// 2 is the missing number in the range since it does not appear in nums.
    /// Example 2:
    /// ---------
    /// Input: nums = [0,1]
    /// Output: 2
    /// Explanation: n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 
    /// 2 is the missing number in the range since it does not appear in nums.
    /// Example 3:

    /// Input: nums = [9,6,4,2,3,5,7,0,1]
    /// Output: 8
    /// Explanation: n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 
    /// 8 is the missing number in the range since it does not appear in nums.
    ///
    /// Constraints:
    /// ------------
    /// n == nums.length
    /// 1 <= n <= 104
    /// 0 <= nums[i] <= n
    /// All the numbers of nums are unique.
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ans: Vec<i32> = vec![-1; nums.len() + 1];
        for i in 0..nums.len() {
            ans[nums[i] as usize] = nums[i];
        }
        
        for i in 0..ans.len() {
            if ans[i] == -1 {
                return i as i32;
            }
        }
        unimplemented!()
    }

    /// ## 713. Subarray Product Less Than K
    ///
    /// Given an array of integers nums and an integer k, return the number of contiguous subarrays 
    /// where the product of all the elements in the subarray is strictly less than k.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [10,5,2,6], k = 100
    /// Output: 8
    /// Explanation: The 8 subarrays that have product less than 100 are:
    /// [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]
    /// Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [1,2,3], k = 0
    /// Output: 0
    ///
    /// Constraints:
    ///
    /// 1 <= nums.length <= 3 * 104
    /// 1 <= nums[i] <= 1000
    /// 0 <= k <= 106
    /// 
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        unimplemented!()   
    }
    
    /// ## 2270. Number of Ways to Split Array
    /// https://leetcode.com/problems/number-of-ways-to-split-array/
    ///
    /// You are given a 0-indexed integer array nums of length n.
    ///
    /// nums contains a valid split at index i if the following are true:
    ///
    /// The sum of the first i + 1 elements is greater than or equal to the sum of the last n - i - 1 elements.
    /// There is at least one element to the right of i. That is, 0 <= i < n - 1.
    /// Return the number of valid splits in nums.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [10,4,-8,7]
    /// Output: 2
    /// Explanation: 
    /// There are three ways of splitting nums into two non-empty parts:
    /// - Split nums at index 0. Then, the first part is [10], and its sum is 10. The second part is [4,-8,7], and its sum is 3. Since 10 >= 3, i = 0 is a valid split.
    /// - Split nums at index 1. Then, the first part is [10,4], and its sum is 14. The second part is [-8,7], and its sum is -1. Since 14 >= -1, i = 1 is a valid split.
    /// - Split nums at index 2. Then, the first part is [10,4,-8], and its sum is 6. The second part is [7], and its sum is 7. Since 6 < 7, i = 2 is not a valid split.
    /// Thus, the number of valid splits in nums is 2.
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [2,3,1,0]
    /// Output: 2
    /// Explanation: 
    /// There are two valid splits in nums:
    /// - Split nums at index 1. Then, the first part is [2,3], and its sum is 5. The second part is [1,0], and its sum is 1. Since 5 >= 1, i = 1 is a valid split. 
    /// - Split nums at index 2. Then, the first part is [2,3,1], and its sum is 6. The second part is [0], and its sum is 0. Since 6 >= 0, i = 2 is a valid split.
    ///
    /// Constraints:
    /// -----------
    /// 2 <= nums.length <= 105
    /// -105 <= nums[i] <= 105
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        let mut prefix_sum = vec![0; size];
        prefix_sum[0] = nums[0];
        for i in 1..size {
            prefix_sum[i] = prefix_sum[i-1] + nums[i];
        }

        let mut ans = 0;

        for j in 0..(size-1) {
            let left = prefix_sum[j];
            let right = prefix_sum[size-1] - left;
            if left >= right {
                ans += 1;
            }
        }

        ans
    }

    /// ## 340. Longest Substring with At Most K Distinct Characters
    /// https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/description/
    /// Given a string s and an integer k, return the length of the longest substring of s that 
    /// contains at most k distinct characters.
    ///
    /// Example 1:
    /// ----------
    /// Input: s = "eceba", k = 2
    /// Output: 3
    /// Explanation: The substring is "ece" with length 3.
    ///
    /// Example 2:
    /// ----------
    /// Input: s = "aa", k = 1
    /// Output: 2
    /// Explanation: The substring is "aa" with length 2.
    ///
    /// Constraints:
    /// ------------
    /// 1 <= s.length <= 5 * 104
    /// 0 <= k <= 50
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        unimplemented!()
    }

    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let s: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if s[i] == '(' {
                stack.push(0);
            } else {
                // pop the previous score
                let prev_score = stack.pop().unwrap();
                // The min score on observing '(' is 1
                let length  = stack.len() ;
                stack[length - 1] += std::cmp::max(prev_score * 2, 1);
            }
        }
        stack.pop().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_averages() {
        let nums = vec![7,4,3,9,1,8,5,2,6];
        let k = 3;
        let res = vec![-1,-1,-1,5,4,4,-1,-1,-1];
        assert_eq!(Solution::get_averages(nums, k), res);
        
        let nums = vec![100000];
        let k = 0;
        let res = vec![100000];
        assert_eq!(Solution::get_averages(nums, k), res);

        let nums = vec![8];
        let k = 100000;
        let res = vec![-1];  
        assert_eq!(Solution::get_averages(nums, k), res);
    }
    #[test]
    fn test_running_sum() {
        let nums = vec![1,2,3,4];
        let res = Solution::running_sum(nums);
        assert_eq!(res, vec![1,3,6,10]);

        let nums = vec![1,1,1,1,1];
        let res = Solution::running_sum(nums);
        assert_eq!(res, vec![1,2,3,4,5]);

        let nums = vec![3,1,2,10,1];
        let res = Solution::running_sum(nums);
        assert_eq!(res, vec![3,4,6,16,17]);
    }

    #[test]
    fn test_min_start_value() {
        let nums = vec![-3,2,-3,4,2];
        let res = 5;
        let output = Solution::min_start_value(nums);
        print!("Result: {}", output);
        assert_eq!(output, res);

        let nums = vec![1,2];
        let res = 1;
        assert_eq!(Solution::min_start_value(nums), res);

        let nums = vec![1,-2,-3];
        let res = 5;
        assert_eq!(Solution::min_start_value(nums), res);
    }

    #[test]
    fn test_check_if_pangram() {
        let sentence = "thequickbrownfoxjumpsoverthelazydog".to_string();
        let res = Solution::check_if_pangram(sentence);
        assert_eq!(res, true);

        let sentence = "leetcode".to_string();
        let res = Solution::check_if_pangram(sentence);
        assert_eq!(res, false);
    }

    #[test]
    fn test_missing_number() {
        let nums = vec![3,0,1];
        let res = 2;
        assert_eq!(Solution::missing_number(nums), res);

        let nums = vec![0,1];
        let res = 2;
        assert_eq!(Solution::missing_number(nums), res);

        let nums = vec![9,6,4,2,3,5,7,0,1];
        let res = 8;
        assert_eq!(Solution::missing_number(nums), res);

    }

    #[test]
    fn test_ways_to_split_array() {
        let nums = vec![10,4,-8,7];
        let res = 2;
        assert_eq!(Solution::ways_to_split_array(nums), res);

        let nums = vec![2,3,1,0];
        let res = 2;
        assert_eq!(Solution::ways_to_split_array(nums), res);

    }

}