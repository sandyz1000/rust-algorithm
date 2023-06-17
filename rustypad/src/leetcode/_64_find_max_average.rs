#![allow(unused)]

struct Solution;

impl Solution {
    /// ## 643. Maximum Average Subarray I
    /// You are given an integer array nums consisting of n elements, and an integer k.
    ///
    /// Find a contiguous subarray whose length is equal to k that has the maximum average 
    /// value and return this value. Any answer with a calculation error less than 10-5 will be accepted.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [1,12,-5,-6,50,3], k = 4
    /// Output: 12.75000
    /// Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [5], k = 1
    /// Output: 5.00000
    ///
    /// Constraints:
    /// ------------
    /// n == nums.length
    /// 1 <= k <= n <= 105
    /// -104 <= nums[i] <= 104
    /// 
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut curr: i32 = 0;
        let k: usize = k as usize;
        // Add all num till k to curr
        for i in 0..k {
            curr += nums[i];
        }
        let mut ans = curr as f64 / k as f64;
        // Add remoaining element
        for i in k..nums.len() {
            curr = curr + nums[i] - nums[(i - k)];
            ans = ans.max(curr as f64 / k as f64);
        }
        
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let nums = vec![0,4,0,3,2];
        let k = 1;
        assert_eq!(Solution::find_max_average(nums, k), 4.00000);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1, 3];
        let expected_output = 2.0;
        assert_eq!(Solution::find_max_average(nums1, 2), expected_output);
    }
}