#![allow(unused)]

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    /// ## 215. Kth Largest Element in an Array
    ///
    /// https://leetcode.com/problems/kth-largest-element-in-an-array/description/
    /// https://www.enjoyalgorithms.com/blog/find-the-kth-smallest-element-in-an-array
    ///
    ///
    /// Given an integer array nums and an integer k, return the kth largest element in the array.
    ///
    /// Note that it is the kth largest element in the sorted order, not the kth distinct element.
    ///
    /// You must solve it in O(n) time complexity.
    ///
    /// Example 1:
    /// ----------
    /// Input: nums = [3,2,1,5,6,4], k = 2
    /// Output: 5
    ///
    /// Example 2:
    /// ----------
    /// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
    /// Output: 4
    ///
    /// Constraints:
    /// ------------
    /// 1 <= k <= nums.length <= 105
    /// -104 <= nums[i] <= 104
    /// 
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        // let mut heap: BinaryHeap<i32> = nums.iter().map(|x| *x).collect();
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
        let mut ans: i32 = 0;
        while k > 0 {
            ans = heap.pop().unwrap();
            k -= 1;    
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        assert_eq!(Solution::find_kth_largest(nums, k), 5);

        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), 4);
    }
}