#![allow(unused)]
use std::{collections::HashMap, hash::Hash};

struct Solution;


impl Solution {
    /// ## 309. Best Time to Buy and Sell Stock with Cooldown
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
    ///
    /// You are given an array prices where prices[i] is the price of a given stock on the ith day.
    ///
    /// Find the maximum profit you can achieve. You may complete as many transactions as you like 
    /// (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
    ///
    /// After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
    /// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock 
    /// before you buy again).
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let prices = vec![1,2,3,0,2];
    /// assert_eq!(Solution::max_profit(prices), 3);
    /// ```
    /// Explanation: transactions = [buy, sell, cooldown, buy, sell]
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let prices = vec![1];
    /// assert_eq!(Solution::max_profit(prices), 0);
    /// ```
    /// Constraints:
    /// -----------
    /// 1 <= prices.length <= 5000
    /// 0 <= prices[i] <= 1000
    pub fn max_profit_with_cooldown(prices: Vec<i32>) -> i32 {
        unimplemented!()
    }

    /// ## Best Time to Buy and Sell Stock with Transaction Fee
    ///
    /// You are given an array prices where prices[i] is the price of a given stock on the ith day, and an 
    /// integer fee representing a transaction fee.
    ///
    /// Find the maximum profit you can achieve. You may complete as many transactions as you like, but you 
    /// need to pay the transaction fee for each transaction.
    ///
    /// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before 
    /// you buy again).
    ///
    /// Example 1:
    /// ----------
    /// let prices = vec![1,3,2,8,4,9]; let fee = 2;
    /// assert_eq!(Solution::max_profit_with_transaction_fee(prices, fee), 8);
    /// ```
    /// Explanation: The maximum profit can be achieved by:
    /// - Buying at prices[0] = 1
    /// - Selling at prices[3] = 8
    /// - Buying at prices[4] = 4
    /// - Selling at prices[5] = 9
    /// The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
    ///
    /// Example 2:
    /// ----------
    /// let prices = vec![1,3,7,5,10,3]; let fee = 3;
    /// assert_eq!(Solution::max_profit_with_transaction_fee(prices, fee), 6);
    /// ```
    /// 
    /// Constraints:
    /// -----------
    /// - 1 <= prices.length <= 5 * 104
    /// - 1 <= prices[i] < 5 * 104
    /// - 0 <= fee < 5 * 104
    pub fn max_profit_with_transaction_fee(prices: Vec<i32>, fee: i32) -> i32 {
        unimplemented!()
    }

    /// ## 746. Min Cost Climbing Stairs
    /// https://leetcode.com/problems/min-cost-climbing-stairs/description/
    ///
    /// You are given an integer array cost where cost[i] is the cost of ith step on a staircase. 
    /// Once you pay the cost, you can either climb one or two steps.
    ///
    /// You can either start from the step with index 0, or the step with index 1.
    ///
    /// Return the minimum cost to reach the top of the floor.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let cost = vec![10,15,20];
    /// assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
    /// ```
    ///
    /// Explanation: You will start at index 1.
    /// - Pay 15 and climb two steps to reach the top.The total cost is 15.
    ///
    /// Example 2:
    /// ---------
    /// ```
    /// let cost = vec![1,100,1,1,1,100,1,1,100,1];
    /// assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    /// ```
    ///
    /// Explanation: You will start at index 0.
    /// - Pay 1 and climb two steps to reach index 2.
    /// - Pay 1 and climb two steps to reach index 4.
    /// - Pay 1 and climb two steps to reach index 6.
    /// - Pay 1 and climb one step to reach index 7.
    /// - Pay 1 and climb two steps to reach index 9.
    /// - Pay 1 and climb one step to reach the top.
    /// The total cost is 6.
    ///
    ///
    /// Constraints:
    /// -----------
    /// * 2 <= cost.length <= 1000
    /// * 0 <= cost[i] <= 999
    ///
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        fn func(pos: i32, cost: &Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
            // Base case
            if pos <= 1 {
                return 0;
            }

            if cache.contains_key(&pos) {
                return cache[&pos];
            }
            let ans = cost[(pos -1) as usize]+ std::cmp::min(func(pos-1, cost, cache), func(pos-2, cost, cache));
            cache.insert(pos, ans);
            cache[&pos]
        }

        fn func2(pos: i32, cost: &Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
            // Base case
            if pos >= cost.len() as i32 {
                return 0;
            }

            if cache.contains_key(&pos) {
                return cache[&pos];
            }

            let ans = cost[pos as usize] + std::cmp::min(func2(pos+1, cost, cache), func2(pos+2, cost, cache));
            cache.insert(pos, ans);
            cache[&pos]
        }

        let mut cache: HashMap<i32, i32> = HashMap::new();
        // func(cost.len() as i32, &cost, &mut cache)      
        func2(0, &cost, &mut cache).min(func2(1, &cost, &mut cache))
    }

    /// ## 70. Climbing Stairs
    /// https://leetcode.com/problems/climbing-stairs/
    ///
    /// You are climbing a staircase. It takes n steps to reach the top.
    ///
    /// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let n = 2; 
    /// assert_eq!(Solution::climb_stairs(n), 2);
    /// ```
    /// Explanation: There are two ways to climb to the top.
    /// 1. 1 step + 1 step
    /// 2. 2 steps
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let n = 3;
    /// assert_eq!(Solution::climb_stairs(n), 3);
    /// ```
    /// Explanation: There are three ways to climb to the top.
    /// 1. 1 step + 1 step + 1 step
    /// 2. 1 step + 2 steps
    /// 3. 2 steps + 1 step
    ///
    /// Constraints:
    /// -----------
    /// 1 <= n <= 45
    ///
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memoizer: HashMap<i32, i32> = HashMap::new();
        fn f(curr_sum: i32, n: i32, memoizer: &mut HashMap<i32, i32>) -> i32 {
            if n == curr_sum {
                return 1;
            } 
            if curr_sum > n {
                return 0;
            }
            
            if memoizer.contains_key(&curr_sum) {
                return memoizer[&curr_sum];
            }

            let ans = f(curr_sum + 1, n, memoizer) + f(curr_sum + 2, n, memoizer);
            memoizer.insert(curr_sum, ans);
            ans
        };

        f(0, n, &mut memoizer)
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
    /// ```
    /// let coins = vec![1,2,5]; let amount = 11;
    /// assert_eq!(Solution::coin_change(coins, amount), 3);
    /// ```
    /// - Explanation: 11 = 5 + 5 + 1
    /// 
    /// Example 2:
    /// ----------
    /// ```
    /// let coins = vec![2]; let amount = 3;
    /// assert_eq!(Solution::coin_change(coins, amount), -1);
    /// ```
    /// Example 3:
    /// ----------
    /// ```
    /// let coins = vec![1]; let amount = 0;
    /// assert_eq!(Solution::coin_change(coins, amount), 0);
    /// ```
    /// Constraints:
    /// -----------
    /// - 1 <= coins.length <= 12
    /// - 1 <= coins[i] <= 231 - 1
    /// - 0 <= amount <= 104
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        fn func(coins: &Vec<i32>, amount: i32, cache: &mut HashMap<i32, i32>) -> i32 {
            if amount == 0 {
                return 0;
            }
            if cache.contains_key(&amount) {
                return cache[&amount];
            }
            
            let mut ans = i32::MAX;
            
            for i in (0..coins.len()).rev() {
                if coins[i] < amount {
                    ans = ans.min(1 + func(coins, amount - coins[i], cache));
                }
            }
            
            ans
        }
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let ans = func(&coins, amount, &mut cache);
        if ans == i32::MAX { -1 } else {ans}
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
    /// ```
    /// let nums = vec![10,9,2,5,3,7,101,18];
    /// assert_eq!(Solution::length_of_lis(nums), 4);
    /// ```
    /// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
    /// 
    /// Example 2:
    /// -----------
    /// ```
    /// let nums = vec![0,1,0,3,2,3];
    /// assert_eq!(Solution::length_of_lis(nums), 4);
    /// ```
    /// Example 3:
    /// -----------
    /// ```
    /// let nums = vec![7,7,7,7,7,7,7];
    /// assert_eq!(Solution::length_of_lis(nums), 1);
    /// ```
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
    /// ```
    /// let s = "leetcode".to_string(); 
    /// let wordDict = vec!["leet","code"].iter().map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::word_break(s, wordDict), true);
    /// ```
    /// Explanation: Return true because "leetcode" can be segmented as "leet code".
    ///  
    /// Example 2:
    /// ----------
    /// ```
    /// let s = "applepenapple".to_string();
    /// let wordDict = vec!["apple","pen"].iter().map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::word_break(s, wordDict), true);
    /// ```
    /// Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
    /// Note that you are allowed to reuse a dictionary word.
    ///  
    /// Example 3:
    /// ----------
    /// ````
    /// let s = "catsandog".to_string(); 
    /// let wordDict = ["cats","dog","sand","and","cat"].iter().map(|x| x.to_string()).collect();
    /// assert_eq!(Solution::word_break(s, wordDict), false);
    /// ```
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
    /// Example 1:
    /// ----------
    /// ```
    /// let nums = vec![1,2,3,1];
    /// assert_eq!(rob(nums), 4);
    /// ```
    /// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
    /// Total amount you can rob = 1 + 3 = 4.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![2,7,9,3,1];
    /// assert_eq!(rob(nums), 12);
    /// ```
    /// Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
    /// Total amount you can rob = 2 + 9 + 1 = 12.
    ///
    /// Constraints:
    /// -----------
    /// - 1 <= nums.length <= 100
    /// - 0 <= nums[i] <= 400
    ///
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn fn_rob(idx: i32, nums: &Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
            // Base case
            if idx >= nums.len() as i32 {
                return 0;
            }
            // Return if this position has already calculated before
            if cache.contains_key(&idx) {
                return cache[&idx];
            }

            // Either rob the current house and skip next house or directly skip current house
            let current = nums[idx as usize] + fn_rob(idx + 2, nums, cache);
            let cost = current.max(fn_rob(idx+1, nums, cache));

            cache.insert(idx, cost);
            cache[&idx]
        }
        let mut cache: HashMap<i32, i32> = HashMap::new();
        
        fn_rob(0, &nums, &mut cache) 
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
    /// ```
    /// let nums = vec![2,3,2];
    /// assert_eq!(Solution::rob_ii(nums), 3);
    /// ```
    /// Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), 
    /// because they are adjacent houses.
    ///
    /// Example 2:
    /// -----------
    /// ```
    /// let nums = vec![1,2,3,1];
    /// assert_eq!(Solution::rob_ii(nums), 4);
    /// ```
    /// Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
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

        // TODO: Fix other test cases
        fn fn_rob(idx: i32, nums: &Vec<i32>, cache: &mut HashMap<i32, i32>) -> i32 {
            // Base case
            if idx >= nums.len() as i32 {
                return 0;
            }

            // If the last house is the first house, since the houses are arrange in circle
            if idx != 0 && nums[idx as usize] == nums[0] {
                return 0;
            }

            // Return if this position has already calculated before
            if cache.contains_key(&idx) {
                return cache[&idx];
            }

            // Either rob the current house and skip next house or directly skip current house
            let current = nums[idx as usize] + fn_rob(idx + 2, nums, cache);
            let cost = current.max(fn_rob(idx+1, nums, cache));

            cache.insert(idx, cost);
            cache[&idx]
        }
        let mut cache: HashMap<i32, i32> = HashMap::new();
        
        fn_rob(0, &nums, &mut cache) 
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
    /// ```
    /// let candidates = vec![2,3,6,7]; let target = 7;
    /// let res = vec![vec![2,2,3], vec![7]];
    /// assert_eq!(combination_sum(candidates, target), res);
    /// ```
    /// Explanation:
    /// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
    /// 7 is a candidate, and 7 = 7.
    /// These are the only two combinations.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let candidates = vec![2,3,5]; let target = 8;
    /// let ans = vec![vec![2,2,2,2], vec![[2,3,3], vec![3,5]];
    /// assert_eq!(combination_sum(candidates, target), ans);
    /// ```
    /// 
    /// Example 3:
    /// ----------
    /// ```
    /// let candidates = [2]; let target = 1;
    /// assert_eq!(combination_sum(candidates, target), vec![]);
    /// ```
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

    /// ## 55. Jump Game
    /// https://leetcode.com/problems/jump-game/
    ///
    /// You are given an integer array nums. You are initially positioned at the array's first index, and 
    /// each element in the array represents your maximum jump length at that position.
    ///
    /// Return true if you can reach the last index, or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let nums = [2,3,1,1,4]
    /// assert_eq!(Solution::can_jump(nums), true);
    /// ```
    /// Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
    ///
    /// Example 2:
    /// ---------
    /// ```
    /// let nums = vec![3,2,1,0,4];
    /// assert_eq!(Solution::can_jump(nums), false);
    /// ```
    // Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which 
    /// makes it impossible to reach the last index.
    ///
    ///
    /// Constraints:
    /// ------------
    /// * 1 <= nums.length <= 104
    /// * 0 <= nums[i] <= 105
    pub fn can_jump(nums: Vec<i32>) -> bool {
        unimplemented!()   
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump() {
        let nums = vec![3,2,1,0,4];
        assert_eq!(Solution::can_jump(nums), false);

    }

    #[test]
    fn test_climb_stairs() {
        let n = 2;
        assert_eq!(Solution::climb_stairs(n), 2);

        let n = 3;
        assert_eq!(Solution::climb_stairs(n), 3);
    }

    #[test]
    fn test_word_break() {
        let s = "leetcode".to_string();
        let word_dict: Vec<String> = 
            vec!["leet","code"]
            .iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::word_break(s, word_dict), true);
    }

    #[test]
    fn test_min_cost_climb_stairs() {
        let cost = vec![10,15,20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);

        let cost = vec![1,100,1,1,1,100,1,1,100,1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }

    #[test]
    fn test_rob() {
        let nums = vec![1,2,3,1];
        assert_eq!(Solution::rob(nums), 4);

        let nums = vec![2,7,9,3,1];
        assert_eq!(Solution::rob(nums), 12);
    }
    
    #[test]
    fn test_rob_ii(){
        let nums = vec![2,3,2];
        assert_eq!(Solution::rob_ii(nums), 3);

        let nums = vec![1,2,3,1];
        assert_eq!(Solution::rob_ii(nums), 4);
        
        // TODO: Fix this test case
        let nums = vec![1,2,3];
        assert_eq!(Solution::rob_ii(nums), 3);
    }
}
