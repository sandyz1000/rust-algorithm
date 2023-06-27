
#![allow(unused)]
struct Solution;

impl Solution {
    /// ## Max Consecutive Ones III
    /// https://leetcode.com/problems/max-consecutive-ones-iii/description/
    /// 
    /// Given a binary array nums and an integer k, return the maximum number of consecutive 1's in 
    /// the array if you can flip at most k 0's.
    ///  
    /// Example 1:
    /// ----------
    /// Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
    /// Output: 6
    /// Explanation: [1,1,1,0,0,1,1,1,1,1,1]
    /// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
    /// 
    /// Example 2:
    /// ----------
    /// Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
    /// Output: 10
    /// Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
    /// Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
    /// 
    /// Constraints:
    ///
    /// 1 <= nums.length <= 105
    /// nums[i] is either 0 or 1.
    /// 0 <= k <= nums.length
    ///
    /// Algorithm:
    /// ----------
    /// Initialize a left, right pointer both point at index 0
    /// Set the max flip count = 0
    ///
    /// if current == 0; then 
    ///     count += 1
    /// 
    /// while count > k; then move the left pointer
    ///     left -= 1
    ///     if current == 0; then count -= 1
    /// Move the right pointer and take the diff = (right - left + 1)
    /// 
    fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut diff: i32 = 0;
        let mut left = 0;
        let mut right = 0;
        let mut count = 0;
        while right < nums.len() {
            if nums[right] == 0 {
                count += 1;
            }
            while count > k {
                if nums[left] == 0 {
                    count -= 1;
                }
                left += 1;
            }
            diff = diff.max((right - left + 1) as i32);
            right += 1;
        }

        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        let nums = vec![1,1,1,0,0,0,1,1,1,1,0];
        let k = 2;
        assert_eq!(Solution::longest_ones(nums, k), 6);
    }

    #[test]
    fn test_2() {
        let nums = vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1];
        let k = 3;
        assert_eq!(Solution::longest_ones(nums, k), 10);
    }
}