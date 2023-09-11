#![allow(unused)]

use core::num;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    /// 1. Two Sum
    ///
    /// Given an array of integers nums and an integer target, return indices of the two numbers such 
    /// that they add up to target.
    ///
    /// You may assume that each input would have exactly one solution, and you may not use the same 
    /// element twice.
    ///
    /// You can return the answer in any order.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let nums = vec![2,7,11,15]; let target = 9;
    /// assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    /// ```
    /// - Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    ///
    /// Example 2:
    /// -----------
    /// ```
    /// let nums = vec![3,2,4]; let target = 6;
    /// assert_eq!(Solution::two_sum(nums, target), vec![1,2]);
    /// ```
    /// 
    /// Example 3:
    /// ----------
    /// ```
    /// let nums = vec![3,3]; let target = 6;
    /// assert_eq!(Solution::two_sum(nums, target), vec![0,1]);
    /// ```
    ///
    /// Constraints:
    /// -----------
    /// * 2 <= nums.length <= 104
    /// * -109 <= nums[i] <= 109
    /// * -109 <= target <= 109
    /// Only one valid answer exists.
    ///
    /// Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?
    /// - Iterate nums and check if target - num contain in hashmap
    /// - return indices of two number
    ///
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            // Look for complement in the hashmap
            let prev = target - *num;
            if hm.contains_key(&prev) {
                return vec![hm[&prev] as i32, index as i32];
            }

            hm.insert(*num, index);
        }
        vec![]
    }

    /// - Two pointer O(n*logn)
    /// - Sort the vector and then take left and right pointer
    /// - Take the current sum and check if it is < target or > target
    /// - If it is < target, move left pointer to the next number
    /// - If it is > target, move right pointer to the next number
    /// - return indices of two number
    fn two_sum_two_pointer(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Equivalent python code
        // num_indices = list(map(lambda x: (x[0], x[1]), enumerate(nums)))
        // nums_indices = sorted(num_indices, key=lambda x: x[1])
        let mut nums_indices: Vec<(usize, &i32)> = nums.iter().enumerate().collect();
        // nums_indices.sort_by_key(|&(_, num)| num);
        nums_indices.sort_by(|x, y| x.1.cmp(y.1));
        let mut left: i32 = 0;
        let mut right: i32 = (nums_indices.len() - 1) as i32;
        while left < right {
            let current = nums_indices[left as usize].1 + nums_indices[right as usize].1;
            if current == target {
                let (l, r) = (nums_indices[left as usize].0, nums_indices[right as usize].0);
                return vec![l as i32, r as i32];
            }
            if current < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }

    /// ## 121. Best Time to Buy and Sell Stock
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
    ///
    /// You are given an array prices where prices[i] is the price of a given stock on the ith day.
    ///
    /// You want to maximize your profit by choosing a single day to buy one stock and choosing a different 
    /// day in the future to sell that stock.
    ///
    /// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
    ///
    ///
    /// Example 1:
    /// -----------
    /// - Input: prices = [7,1,5,3,6,4]
    /// - Output: 5
    /// - Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
    /// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
    ///
    /// Example 2:
    /// -----------
    /// - Input: prices = [7,6,4,3,1]
    /// - Output: 0
    /// - Explanation: In this case, no transactions are done and the max profit = 0.
    ///
    ///
    /// Constraints:
    /// -----------
    /// 1 <= prices.length <= 105
    /// 0 <= prices[i] <= 104
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut max_profit = i32::MIN;

        for price in prices {
            min_price = min_price.min(price);
            
            // Calculate the max_profit
            max_profit = max_profit.max(price - min_price);
        }
        max_profit
    }
    
    /// ## Best Time to Buy and Sell Stock IV
    /// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/ 
    /// https://medium.com/tech-pulse/an-easier-way-to-solve-all-stock-trading-questions-on-leetcode-fd081421f081
    ///
    /// You are given an integer array prices where prices[i] is the price of a given stock on the ith day, 
    /// and an integer k.
    ///
    /// Find the maximum profit you can achieve. You may complete at most k transactions: i.e. you may buy 
    /// at most k times and sell at most k times.
    ///
    /// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before 
    /// you buy again).
    ///
    /// Example 1:
    /// -----------
    /// Input: k = 2, prices = [2,4,1]
    /// Output: 2
    /// Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
    ///
    /// Example 2:
    ///
    /// Input: k = 2, prices = [3,2,6,5,0,3]
    /// Output: 7
    /// Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4. Then buy on 
    /// day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= k <= 100
    /// 1 <= prices.length <= 1000
    /// 0 <= prices[i] <= 1000
    ///
    pub fn max_profit_iv(prices: Vec<i32>) -> i32 {
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
        // Use hashset to add value and return false if item exist in hashset
        let mut hashset: HashSet<i32> = HashSet::new();
        for num in nums {
            if hashset.contains(&num) {
                return true;
            }
            hashset.insert(num);
        }
        false
    }

    fn contains_duplicate_ii(nums: Vec<i32>) -> bool {
        let duplicates: HashSet<i32> = nums.iter().map(|x| *x).collect();
        duplicates.len() != nums.len()
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
        let mut res = vec![];
        let mut prefix = 1;

        for i in 0..nums.len() {
            res.push(prefix);
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }
        
        res
    }
    
    /// ## 152. Maximum Product Subarray
    ///
    /// Given an integer array nums, find a subarray that has the largest product, and return the product.
    ///
    /// The test cases are generated so that the answer will fit in a 32-bit integer.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let nums = vec![2,3,-2,4];
    /// assert_eq!(Solution::max_product(nums), 6);
    /// ```
    /// Explanation: \[2,3] has the largest product 6.
    ///
    /// Example 2:
    /// -----------
    /// ```
    /// let nums = vec![-2,0,-1];
    /// assert_eq!(Solution::max_product(nums), 0);
    /// ```
    ///
    /// Explanation: The result cannot be 2, because \[-2,-1] is not a subarray.
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= nums.length <= 2 * 104
    /// * -10 <= nums\[i] <= 10
    /// * The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
    pub fn max_product(nums: Vec<i32>) -> i32 {
        unimplemented!( )    
    }

    /// 53. Maximum Subarray
    ///
    /// Given an integer array nums, find the subarray with the largest sum, and return its sum.
    ///
    /// Example 1:
    /// -----------
    /// ```
    /// let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    /// assert_eq!(Solution::max_sub_array(nums), 6);
    /// ```
    /// - Explanation: The subarray \[4,-1,2,1] has the largest sum 6.
    ///
    /// Example 2:
    /// -----------
    /// ```
    /// let nums = vec![1];
    /// assert_eq!(Solution::max_sub_array(nums), 1);
    /// ```
    /// - Explanation: The subarray \[1] has the largest sum 1.
    ///
    /// Example 3:
    /// -----------
    /// ```
    /// let nums = vec![5,4,-1,7,8];
    /// assert_eq!(Solution::max_sub_array(nums), 23);
    /// ```
    /// - Explanation: The subarray \[5,4,-1,7,8] has the largest sum 23.
    ///
    ///
    /// Constraints:
    /// -----------
    /// * 1 <= nums.length <= 105
    /// * -104 <= nums\[i] <= 104
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
    /// ```
    /// let nums = vec![-1,0,1,2,-1,-4];
    /// assert_eq!(Solution::three_sum(nums), vec![vec![-1,-1,2],vec![-1,0,1]]);
    /// ```
    /// Explanation: 
    /// - nums\[0] + nums\[1] + nums\[2] = (-1) + 0 + 1 = 0.
    /// - nums\[1] + nums\[2] + nums\[4] = 0 + 1 + (-1) = 0.
    /// - nums\[0] + nums\[3] + nums\[4] = (-1) + 2 + (-1) = 0.
    /// The distinct triplets are \[-1,0,1] and \[-1,-1,2].
    /// Notice that the order of the output and the order of the triplets does not matter.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let nums = vec![0,1,1];
    /// assert_eq!(Solution::three_sum(nums), vec![vec![]]);
    /// ```
    /// Explanation: The only possible triplet does not sum up to 0.
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let nums = vec![0,0,0];
    /// let ans = vec![vec![0,0,0]];
    /// assert_eq!(Solution::three_sum(nums), ans);
    /// ```
    /// Explanation: The only possible triplet sums up to 0.
    ///
    /// Constraints:
    /// ------------
    /// * 3 <= nums.length <= 3000
    /// * -105 <= nums[i] <= 105
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn two_sum(nums: &Vec<i32>, start: i32, ans: &mut Vec<Vec<i32>>) {
            let mut left: i32 = start + 1;
            let mut right: i32 = nums.len() as i32 - 1;
            while left < right {
                let total = nums[left as usize] + nums[right as usize] + nums[start as usize];
                if total == 0 {
                    ans.push(vec![nums[start as usize], nums[left as usize], nums[right as usize]]);
                    left += 1;
                    right -= 1;
                    while left < right && nums[left as usize] == nums[(left - 1) as usize] {
                        left += 1;
                    }
                    continue;
                }

                // If total > 0, decrease the right pointer
                if total > 0 {
                    right -= 1;
                }
                // Else increse the left pointer
                else {
                    left += 1;
                }
            }
        }

        nums.sort();
        if nums.len() < 3 {
            return vec![];
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        for start in 0..nums.len() {
            // After sorting we only look for start value < 0
            if nums[start] > 0 {
                break;
            }
            // nums[start] != nums[start -1] is needed to avoid adding solution
            // that already has been added
            if start == 0 || nums[start] != nums[start -1] {
                two_sum(&nums, start as i32, &mut ans);
            }
        }

        ans
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
        let mut left: i32 = 0;
        let mut right: i32 = height.len() as i32 - 1;
        let mut max_area: i32 = i32::MIN;

        while left < right {
            let length = (right - left);
            let min_height = height[left as usize].min(height[right as usize]);
            max_area = max_area.max(min_height * length);

            if height[left as usize] < height[right as usize] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
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
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) /2;
            // If target found return mid
            if nums[mid] == target {
                return mid as i32;
            }
            // If left side sorted i.e. left < mid
            if nums[left] <= nums[mid] {
                // If target is b/w left and mid, then right = mid -1
                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } 
            else {
                // If target is b/w mid and right, then right = mid -1
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        } 
        -1
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
        if nums.len() == 1 {
            return nums[0]
        }
        let mut right = nums.len() - 1;
        let mut left = 0;

        // Base case if nums[right] > nums[0], 
        // then item is already sorted return nums[0]
        if nums[right] > nums[0] {
            return nums[0];
        }

        while left < right {
            let mid = left + (right - left) / 2;

            // If mid + 1 < mid, then return mid + 1
            if nums[mid + 1] < nums[mid] {
                return nums[(mid +1)];
            }
            // If mid - 1 > mid, then return mid
            if nums[mid - 1] > nums[mid] {
                return nums[mid];
            }

            // If mid > 0 index item, then search right (left is sorted)
            if nums[mid] > nums[0] {
                left = mid +1;
            } else {
                right = mid -1;
            }
        }
        -1
    }

    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let order_map: HashMap<char, i32> = order.chars()
            .enumerate()
            .map(|(i, c)| (c, i as i32)).collect();

        for i in 0..words.len() -1 {
            let word1: Vec<char> = words[i].chars().collect();
            let word2: Vec<char> = words[i+1].chars().collect();
            for pos in 0..word1.len() {

                // If two consecutive word has unequal character and 
                // order is reverse return false, cases where ["apple", "app"]
                if pos >= word2.len() {
                    return false;
                }
                // If the first mismatch character found and 
                // order is right break the loop
                if word1[pos] != word2[pos] {
                    // Also if the second word length < than the first one, return false 
                    if order_map[&word1[pos]] > order_map[&word2[pos]] {
                        return false;
                    }

                    break
                }
            }
        }

        true
    }

}


#[cfg(test)]
mod test {
    use super::*;

    fn test_max_profit() {

    }

    #[test]
    fn test_string_ref() {
        // Rust &str vs String working example for understanding
            
        let foo: &str = "hello";
        let bar: String = foo.to_string();
        let baz: &str = &bar;
        let x: bool = bar.eq(baz);
        
        // This will be a copy of baz string
        let mut foobar: String = baz.to_owned();
        foobar = foobar + " " + "world";
    
        println!("The value of x: {} ", x);
        println!("The value of foo: {} ", foo);
        println!("The value of foobar: {} ", foobar);
    }
    
    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
    
    #[test]
    fn test_two_sum_two_pointer() {
        let nums = vec![3,2,4];
        let target = 6;
        assert_eq!(Solution::two_sum_two_pointer(nums, target), vec![1, 2]);
    }
}