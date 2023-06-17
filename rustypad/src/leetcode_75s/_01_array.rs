#![allow(unused)]

struct Solution;

impl Solution {
    /// ## 121. Best Time to Buy and Sell Stock
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    /// https://medium.com/tech-pulse/an-easier-way-to-solve-all-stock-trading-questions-on-leetcode-fd081421f081
    ///
    /// You are given an array prices where prices[i] is the price of a given stock on the ith day.
    ///
    /// You want to maximize your profit by choosing a single day to buy one stock and choosing a different 
    /// day in the future to sell that stock.
    ///
    /// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
    ///
    ///
    ///
    /// Example 1:
    /// -----------
    /// Input: prices = [7,1,5,3,6,4]
    /// Output: 5
    /// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
    /// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    /// Example 2:
    ///
    /// Input: prices = [7,6,4,3,1]
    /// Output: 0
    /// Explanation: In this case, no transactions are done and the max profit = 0.
    ///
    ///
    /// Constraints:
    ///
    /// 1 <= prices.length <= 105
    /// 0 <= prices[i] <= 104
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        unimplemented!()
    }

    /// ## 217. Contains Duplicate
    /// https://leetcode.com/problems/contains-duplicate/
    /// 
    /// Given an integer array nums, return true if any value appears at least twice in the array, 
    /// and return false if every element is distinct.
    ///
    /// Example 1:
    /// -----------
    /// - Input: nums = [1,2,3,1]
    /// - Output: true
    /// 
    /// Example 2:
    /// -----------
    /// - Input: nums = [1,2,3,4]
    /// - Output: false
    ///
    /// Example 3:
    /// -----------
    /// - Input: nums = [1,1,1,3,3,4,3,2,4,2]
    /// - Output: true
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 105
    /// -109 <= nums[i] <= 109
    /// 
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        unimplemented!()
    }

    /// ## 238. Product of Array Except Self
    /// https://leetcode.com/problems/product-of-array-except-self/
    /// 
    /// Given an integer array nums, return an array answer such that answer[i] is equal to the product of 
    /// all the elements of nums except nums[i].
    ///
    /// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
    ///
    /// You must write an algorithm that runs in O(n) time and without using the division operation.
    ///
    ///
    /// Example 1:
    /// ------------
    /// - Input: nums = [1,2,3,4]
    /// - Output: [24,12,8,6]
    ///
    /// Example 2:
    /// --------------
    /// - Input: nums = [-1,1,0,-3,3]
    /// - Output: [0,0,9,0,0]
    ///
    ///
    /// Constraints:
    /// -----------
    /// 2 <= nums.length <= 105
    /// -30 <= nums[i] <= 30
    /// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
    ///
    /// Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as 
    /// extra space for space complexity analysis.)
    ///
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        unimplemented!()   
    }
    
    /// ## 152. Maximum Product Subarray
    ///
    /// Given an integer array nums, find a subarray that has the largest product, and return the product.
    ///
    /// The test cases are generated so that the answer will fit in a 32-bit integer.
    ///
    /// Example 1:
    /// -----------
    /// - Input: nums = [2,3,-2,4]
    /// - Output: 6
    ///
    /// Explanation: [2,3] has the largest product 6.
    ///
    /// Example 2:
    /// -----------
    /// - Input: nums = [-2,0,-1]
    /// - Output: 0
    ///
    /// Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 2 * 104
    /// -10 <= nums[i] <= 10
    /// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
    pub fn max_product(nums: Vec<i32>) -> i32 {
        unimplemented!( )    
    }

    /// 53. Maximum Subarray
    ///
    /// Given an integer array nums, find the subarray with the largest sum, and return its sum.
    ///
    /// Example 1:
    /// -----------
    /// - Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
    /// - Output: 6
    /// - Explanation: The subarray [4,-1,2,1] has the largest sum 6.
    ///
    /// Example 2:
    /// -----------
    /// - Input: nums = [1]
    /// - Output: 1
    /// - Explanation: The subarray [1] has the largest sum 1.
    ///
    /// Example 3:
    /// -----------
    /// - Input: nums = [5,4,-1,7,8]
    /// - Output: 23
    /// - Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 105
    /// -104 <= nums[i] <= 104
    ///
    ///
    /// Follow up: If you have figured out the O(n) solution, try coding another solution using the 
    /// divide and conquer approach, which is more subtle.
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        unimplemented!( )   
    }

    /// ## 15. 3Sum
    /// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] 
    /// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
    ///
    /// Notice that the solution set must not contain duplicate triplets.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [-1,0,1,2,-1,-4]
    /// - Output: [[-1,-1,2],[-1,0,1]]
    ///
    /// Explanation: 
    /// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
    /// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
    /// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are [-1,0,1] and [-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    ///
    /// Example 2:
    /// ----------
    /// - Input: nums = [0,1,1]
    /// - Output: []
    ///
    /// Explanation: The only possible triplet does not sum up to 0.
    ///
    /// Example 3:
    /// ----------
    /// - Input: nums = [0,0,0]
    /// - Output: [[0,0,0]]
    ///
    /// Explanation: The only possible triplet sums up to 0.
    ///
    /// Constraints:
    /// ------------
    /// 3 <= nums.length <= 3000
    /// -105 <= nums[i] <= 105
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        unimplemented!()  
    }

    /// ## 11. Container With Most Water
    /// https://leetcode.com/problems/container-with-most-water/
    ///
    /// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints 
    /// of the ith line are (i, 0) and (i, height[i]).
    ///
    /// Find two lines that together with the x-axis form a container, such that the container contains the most water.
    ///
    /// Return the maximum amount of water a container can store.
    ///
    /// Notice that you may not slant the container.
    ///
    /// Example 1:
    /// ----------
    /// - Input: height = [1,8,6,2,5,4,8,3,7]
    /// - Output: 49
    /// - Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of 
    /// water (blue section) the container can contain is 49.
    ///
    /// Example 2:
    /// ----------
    /// - Input: height = [1,1]
    /// - Output: 1
    ///
    /// Constraints:
    /// ------------
    /// n == height.length
    /// 2 <= n <= 105
    /// 0 <= height[i] <= 104
    /// 
    pub fn max_area(height: Vec<i32>) -> i32 {
        unimplemented!()
    }
    
    /// ## 33. Search in Rotated Sorted Array
    ///
    /// There is an integer array nums sorted in ascending order (with distinct values).
    ///
    /// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k 
    /// (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], 
    /// nums[0], nums[1], ..., nums[k-1]] (0-indexed). 
    /// For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
    ///
    /// Given the array nums after the possible rotation and an integer target, return the index of 
    /// target if it is in nums, or -1 if it is not in nums.
    ///
    /// You must write an algorithm with O(log n) runtime complexity.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [4,5,6,7,0,1,2], target = 0
    /// - Output: 4
    ///
    /// Example 2:
    /// ----------
    /// - Input: nums = [4,5,6,7,0,1,2], target = 3
    /// - Output: -1
    ///
    /// Example 3:
    /// ----------
    /// - Input: nums = [1], target = 0
    /// - Output: -1
    ///
    ///
    /// Constraints:
    /// ----------
    /// 1 <= nums.length <= 5000
    /// -104 <= nums[i] <= 104
    /// All values of nums are unique.
    /// nums is an ascending array that is possibly rotated.
    /// -104 <= target <= 104
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        unimplemented!()   
    }

    /// ## 153. Find Minimum in Rotated Sorted Array
    ///
    /// Suppose an array of length n sorted in ascending order is rotated between 1 and n times. 
    /// For example, the array nums = [0,1,2,4,5,6,7] might become:
    /// - [4,5,6,7,0,1,2] if it was rotated 4 times.
    /// - [0,1,2,4,5,6,7] if it was rotated 7 times.
    ///
    /// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array 
    /// [a[n-1], a[0], a[1], a[2], ..., a[n-2]].
    ///
    /// Given the sorted rotated array nums of unique elements, return the minimum element of this array.
    ///
    /// You must write an algorithm that runs in O(log n) time.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [3,4,5,1,2]
    /// - Output: 1
    /// - Explanation: The original array was [1,2,3,4,5] rotated 3 times.
    /// 
    /// Example 2:
    /// ----------
    /// - Input: nums = [4,5,6,7,0,1,2]
    /// - Output: 0
    /// - Explanation: The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
    /// 
    /// Example 3:
    /// ----------
    /// - Input: nums = [11,13,15,17]
    /// - Output: 11
    /// - Explanation: The original array was [11,13,15,17] and it was rotated 4 times. 
    ///
    /// Constraints:
    /// -----------
    /// n == nums.length
    /// 1 <= n <= 5000
    /// -5000 <= nums[i] <= 5000
    /// All the integers of nums are unique.
    /// nums is sorted and rotated between 1 and n times.
    ///
    pub fn find_min(nums: Vec<i32>) -> i32 {
        unimplemented!()    
    }


}


#[cfg(test)]
mod test {
    use super::*;

    fn test_max_profit() {

    }
}