#![allow(unused)]

struct Solution;

impl Solution {
    /// ## 70. Climbing Stairs
    /// https://leetcode.com/problems/climbing-stairs/
    ///
    /// You are climbing a staircase. It takes n steps to reach the top.
    ///
    /// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
    ///
    /// Example 1:
    /// ----------
    /// - Input: n = 2
    /// - Output: 2
    /// - Explanation: There are two ways to climb to the top.
    /// 1. 1 step + 1 step
    /// 2. 2 steps
    ///
    /// Example 2:
    /// ----------
    /// - Input: n = 3
    /// - Output: 3
    /// - Explanation: There are three ways to climb to the top.
    /// 1. 1 step + 1 step + 1 step
    /// 2. 1 step + 2 steps
    /// 3. 2 steps + 1 step
    ///
    /// Constraints:
    /// -----------
    /// 1 <= n <= 45
    ///
    pub fn climb_stairs(n: i32) -> i32 {
        unimplemented!()
    }

    /// ## Coin Change
    /// https://leetcode.com/problems/coin-change/
    ///
    /// You are given an integer array coins representing coins of different denominations
    /// and an integer amount representing a total amount of money.
    ///
    /// Return the fewest number of coins that you need to make up that amount. If that amount
    /// of money cannot be made up by any combination of the coins, return -1.
    ///
    /// You may assume that you have an infinite number of each kind of coin.
    ///
    /// Example 1:
    /// ----------
    /// - Input: coins = [1,2,5], amount = 11
    /// - Output: 3
    /// - Explanation: 11 = 5 + 5 + 1
    /// 
    /// Example 2:
    /// ----------
    /// - Input: coins = [2], amount = 3
    /// - Output: -1
    /// 
    /// Example 3:
    /// ----------
    /// - Input: coins = [1], amount = 0
    /// - Output: 0
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= coins.length <= 12
    /// - 1 <= coins[i] <= 231 - 1
    /// - 0 <= amount <= 104
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        unimplemented!()
    }

    /// ## Longest Increasing Subsequence
    ///
    /// https://leetcode.com/problems/longest-increasing-subsequence/
    ///
    /// Given an integer array nums, return the length of the longest strictly increasing 
    /// subsequence.
    ///
    /// Example 1:
    /// ------------
    /// - Input: nums = [10,9,2,5,3,7,101,18]
    /// - Output: 4
    /// - Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    /// 
    /// Example 2:
    /// -----------
    /// - Input: nums = [0,1,0,3,2,3]
    /// - Output: 4
    /// 
    /// Example 3:
    /// -----------
    /// - Input: nums = [7,7,7,7,7,7,7]
    /// - Output: 1
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= nums.length <= 2500
    /// - -104 <= nums[i] <= 104
    ///
    /// Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
    ///
    /// Recurse with start index and prev value
    /// If current > prev; than add + 1 to ans and set prev = current and recurse
    /// If not than skip and compare with the next start index
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        unimplemented!()   
    }

    /// ## 139. Word Break
    ///
    /// https://leetcode.com/problems/word-break/
    ///
    /// Given a string s and a dictionary of strings wordDict, return true if s can be 
    /// segmented into a space-separated sequence of one or more dictionary words.
    ///
    /// Note that the same word in the dictionary may be reused multiple times in the 
    /// segmentation.
    ///
    /// Example 1:
    /// ----------
    /// - Input: s = "leetcode", wordDict = ["leet","code"]
    /// - Output: true
    /// - Explanation: Return true because "leetcode" can be segmented as "leet code".
    ///  
    /// Example 2:
    /// ----------
    /// - Input: s = "applepenapple", wordDict = ["apple","pen"]
    /// - Output: true
    /// - Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
    /// Note that you are allowed to reuse a dictionary word.
    ///  
    /// Example 3:
    /// ----------
    /// - Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
    /// - Output: false
    ///
    /// Constraints:
    /// -----------
    /// 1 <= s.length <= 300
    /// 1 <= wordDict.length <= 1000
    /// 1 <= wordDict[i].length <= 20
    /// s and wordDict[i] consist of only lowercase English letters.
    /// All the strings of wordDict are unique.
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        unimplemented!()
    }

    /// ## 198. House Robber
    /// https://leetcode.com/problems/house-robber/
    ///
    /// You are a professional robber planning to rob houses along a street. Each house has a certain amount of 
    /// money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have 
    /// security systems connected and it will automatically contact the police if two adjacent houses were broken 
    /// into on the same night.
    ///
    /// Given an integer array nums representing the amount of money of each house, return the maximum amount of 
    /// money you can rob tonight without alerting the police.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [1,2,3,1]
    /// - Output: 4
    /// - Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
    /// Total amount you can rob = 1 + 3 = 4.
    ///
    /// Example 2:
    /// ----------
    /// - Input: nums = [2,7,9,3,1]
    /// - Output: 12
    /// - Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
    /// Total amount you can rob = 2 + 9 + 1 = 12.
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= nums.length <= 100
    /// - 0 <= nums[i] <= 400
    ///
    pub fn rob(nums: Vec<i32>) -> i32 {
        unimplemented!()   
    }

    /// ## 213. House Robber II
    /// https://leetcode.com/problems/house-robber-ii/
    ///
    /// You are a professional robber planning to rob houses along a street. Each house has a certain amount of 
    /// money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor 
    /// of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically 
    /// contact the police if two adjacent houses were broken into on the same night.
    ///
    /// Given an integer array nums representing the amount of money of each house, return the maximum amount of 
    /// money you can rob tonight without alerting the police.
    ///
    /// Example 1:
    /// -----------
    /// - Input: nums = [2,3,2]
    /// - Output: 3
    /// - Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
    ///
    /// Example 2:
    /// -----------
    /// - Input: nums = [1,2,3,1]
    /// - Output: 4
    /// - Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
    /// Total amount you can rob = 1 + 3 = 4.
    ///
    /// Example 3:
    /// ----------- 
    /// - Input: nums = [1,2,3]
    /// - Output: 3
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= nums.length <= 100
    /// - 0 <= nums[i] <= 1000
    ///
    pub fn rob_ii(nums: Vec<i32>) -> i32 {
        unimplemented!()
    }

    /// ## Combination Sum
    /// https://leetcode.com/problems/combination-sum/
    /// 
    /// Given an array of distinct integers candidates and a target integer target, return a list of all unique 
    /// combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
    ///
    /// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the 
    /// frequency of at least one of the chosen numbers is different.
    ///
    /// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 
    /// combinations for the given input.
    ///
    /// Example 1:
    /// ----------
    /// - Input: candidates = [2,3,6,7], target = 7
    /// - Output: [[2,2,3],[7]]
    /// - Explanation:
    /// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
    /// 7 is a candidate, and 7 = 7.
    /// These are the only two combinations.
    ///
    /// Example 2:
    /// ----------
    /// - Input: candidates = [2,3,5], target = 8
    /// - Output: [[2,2,2,2],[2,3,3],[3,5]]
    ///
    /// Example 3:
    /// ----------
    /// - Input: candidates = [2], target = 1
    /// - Output: []
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= candidates.length <= 30
    /// - 2 <= candidates[i] <= 40
    /// - All elements of candidates are distinct.
    /// - 1 <= target <= 40
    ///
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climb_stairs() {}

    #[test]
    fn test_word_break() {}
}
