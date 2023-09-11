#![allow(unused)]


struct Solution;

impl Solution {
    /// 704. Binary Search
    /// https://leetcode.com/problems/binary-search/
    ///
    /// Given an array of integers nums which is sorted in ascending order, and an integer target, 
    /// write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
    ///
    /// You must write an algorithm with O(log n) runtime complexity.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [-1,0,3,5,9,12], target = 9
    /// - Output: 4
    /// - Explanation: 9 exists in nums and its index is 4
    ///
    /// Example 2:
    /// ----------
    /// - Input: nums = [-1,0,3,5,9,12], target = 2
    /// - Output: -1
    /// - Explanation: 2 does not exist in nums so return -1
    ///
    /// Constraints:
    /// ------------
    /// 1 <= nums.length <= 104
    /// -104 < nums[i], target < 104
    /// All the integers in nums are unique.
    /// nums is sorted in ascending order.
    ///
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Get the left and right position to search for
        // Get the mid
        let mut left: i32 = 0; let mut right: i32 = (nums.len() - 1) as i32;
        while left <= right {
            let mid: i32 = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            else if nums[mid as usize] > target {
                // Search in the left side
                right = mid - 1;
            } else {
                // Search in the right side
                left = mid + 1;
            }
        }

        -1
    }


    /// ## 74. Search a 2D Matrix
    /// https://leetcode.com/problems/search-a-2d-matrix/
    /// 
    /// You are given an m x n integer matrix matrix with the following two properties:
    ///
    /// Each row is sorted in non-decreasing order.
    /// The first integer of each row is greater than the last integer of the previous row.
    /// Given an integer target, return true if target is in matrix or false otherwise.
    ///
    /// You must write a solution in O(log(m * n)) time complexity.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]]; target = 3;
    /// assert_eq!(Solution::search_matrix(matrix, target), true);
    /// ```
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let matrix = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]]; let target = 13;
    /// assert_eq!(Solution::search_matrix(matrix, target), false);
    /// ```
    /// Constraints:
    /// ------------
    /// * m == matrix.length
    /// * n == matrix\[i].length
    /// * 1 <= m, n <= 100
    /// * -104 <= matrix\[i]\[j], target <= 104
    ///
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n_row = matrix.len() as i32;
        let n_col = matrix[0].len() as i32;
        let mut left = 0; let mut right = n_row * n_col - 1;
        if n_row == 0{
            return false;
        }
        
        while left <= right {
            let mid  = (left + right) / 2;
            let row = (mid / n_col); let pos = (mid % n_col);
            
            if matrix[row as usize][pos as usize] == target {
                return true
            }
            
            if matrix[row as usize][pos as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        false
    }

    /// ## 2300. Successful Pairs of Spells and Potions
    /// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
    ///
    /// You are given two positive integer arrays spells and potions, of length n and m respectively, 
    /// where spells[i] represents the strength of the ith spell and potions[j] represents the strength 
    /// of the jth potion.
    ///
    /// You are also given an integer success. A spell and potion pair is considered successful if the 
    /// product of their strengths is at least success.
    ///
    /// Return an integer array pairs of length n where pairs[i] is the number of potions that will form a 
    /// successful pair with the ith spell.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let spells = vec![5,1,3]; let potions = vec![1,2,3,4,5]; let success = 7;
    /// let ans: Vec<i32> = vec![4,0,3];
    /// assert_eq!(Solution::successful_pairs(spells, potions, success), ans);
    /// ```
    /// Explanation:
    /// - 0th spell: 5 * \[1,2,3,4,5] = \[5,10,15,20,25]. 4 pairs are successful.
    /// - 1st spell: 1 * \[1,2,3,4,5] = \[1,2,3,4,5]. 0 pairs are successful.
    /// - 2nd spell: 3 * \[1,2,3,4,5] = \[3,6,9,12,15]. 3 pairs are successful.
    /// Thus, [4,0,3] is returned.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let spells = vec![3,1,2]; let potions = vec![8,5,8]; let success = 16;
    /// let ans = vec![2,0,2];
    /// ```
    /// Explanation:
    /// - 0th spell: 3 * \[8,5,8] = \[24,15,24]. 2 pairs are successful.
    /// - 1st spell: 1 * \[8,5,8] = \[8,5,8]. 0 pairs are successful. 
    /// - 2nd spell: 2 * \[8,5,8] = \[16,10,16]. 2 pairs are successful. 
    /// Thus, \[2,0,2] is returned.
    ///
    /// Constraints:
    /// ------------
    /// * n == spells.length
    /// * m == potions.length
    /// * 1 <= n, m <= 105
    /// * 1 <= spells[i], potions[i] <= 105
    /// * 1 <= success <= 1010
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        unimplemented!()
    }

    /// ## 35. Search Insert Position
    ///
    /// Given a sorted array of distinct integers and a target value, return the index if the target is found. 
    /// If not, return the index where it would be if it were inserted in order.
    ///
    /// You must write an algorithm with O(log n) runtime complexity.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [1,3,5,6], target = 5
    // - Output: 2
    //
    // Example 2:
    // ----------
    // - Input: nums = [1,3,5,6], target = 2
    // - Output: 1
    //
    // Example 3:
    // ----------
    // Input: nums = [1,3,5,6], target = 7
    // Output: 4
    //
    // Constraints:
    // ------------
    // 1 <= nums.length <= 104
    // -104 <= nums[i] <= 104
    // nums contains distinct values sorted in ascending order.
    // -104 <= target <= 104
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // Get the left and right pointer
        let mut left: i32 = 0; let mut right: i32 = nums.len() as i32 - 1;
        
        // This is required for arr contains 1 value
        while left <= right {
            let mid: i32 = left + (right - left) /2 ;
            
            // If target is found 
            if nums[mid as usize] == target {
                return mid;
            }

            // Search in the left section
            if nums[mid as usize] > target {
                right = mid - 1;
            } 
            // Search in the right section
            else {
                left = mid + 1;
            }
        }
        // If no target found return left
        return left;
    }

    /// ## Longest Subsequence With Limited Sum
    /// https://leetcode.com/problems/longest-subsequence-with-limited-sum/description/
    ///
    /// You are given an integer array nums of length n, and an integer array queries of length m.
    ///
    /// Return an array answer of length m where answer[i] is the maximum size of a subsequence that you can take 
    /// from nums such that the sum of its elements is less than or equal to queries[i].
    ///
    /// A subsequence is an array that can be derived from another array by deleting some or no elements without 
    /// changing the order of the remaining elements.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [4,5,2,1], queries = [3,10,21]
    /// - Output: [2,3,4]
    ///
    /// Explanation: We answer the queries as follows:
    /// - The subsequence [2,1] has a sum less than or equal to 3. It can be proven that 2 is the maximum size of 
    /// such a subsequence, so answer[0] = 2.
    /// - The subsequence [4,5,1] has a sum less than or equal to 10. It can be proven that 3 is the maximum size 
    /// of such a subsequence, so answer[1] = 3.
    /// - The subsequence [4,5,2,1] has a sum less than or equal to 21. It can be proven that 4 is the maximum size 
    /// of such a subsequence, so answer[2] = 4.
    ///
    /// Example 2:
    /// ---------
    /// - Input: nums = [2,3,4,5], queries = [1]
    /// -Output: [0]
    /// Explanation: The empty subsequence is the only subsequence that has a sum less than or equal to 1, so answer[0] = 0.
    ///
    /// Constraints:
    /// ------------
    /// n == nums.length
    /// m == queries.length
    /// 1 <= n, m <= 1000
    /// 1 <= nums[i], queries[i] <= 106
    ///
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        unimplemented!()
    }

    /// ## 875. Koko Eating Bananas
    ///
    /// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. 
    /// The guards have gone and will come back in h hours.
    ///
    /// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of 
    /// bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all 
    /// of them instead and will not eat any more bananas during this hour.
    ///
    /// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
    ///
    /// Return the minimum integer k such that she can eat all the bananas within h hours.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: piles = [3,6,7,11], h = 8
    /// - Output: 4
    ///
    /// Example 2:
    /// ----------
    /// - Input: piles = [30,11,23,4,20], h = 5
    /// - Output: 30
    ///
    /// Example 3:
    /// ----------
    /// - Input: piles = [30,11,23,4,20], h = 6
    /// - Output: 23
    ///
    /// Constraints:
    /// ------------
    /// 1 <= piles.length <= 104
    /// piles.length <= h <= 109
    /// 1 <= piles[i] <= 109
    ///
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        unimplemented!()
    }

    /// ## 1870. Minimum Speed to Arrive on Time
    /// https://leetcode.com/problems/minimum-speed-to-arrive-on-time/
    /// 
    /// You are given a floating-point number hour, representing the amount of time you have to reach the office. 
    /// To commute to the office, you must take n trains in sequential order. You are also given an integer array 
    /// dist of length n, where dist[i] describes the distance (in kilometers) of the ith train ride.
    ///
    /// Each train can only depart at an integer hour, so you may need to wait in between each train ride.
    ///
    /// For example, if the 1st train ride takes 1.5 hours, you must wait for an additional 0.5 hours before you can 
    /// depart on the 2nd train ride at the 2 hour mark.
    /// Return the minimum positive integer speed (in kilometers per hour) that all the trains must travel at for you 
    /// to reach the office on time, or -1 if it is impossible to be on time.
    ///
    /// Tests are generated such that the answer will not exceed 107 and hour will have at most two digits after the decimal point.
    ///
    /// Example 1:
    /// -----------
    /// - Input: dist = [1,3,2], hour = 6
    /// - Output: 1
    ///
    /// Explanation: At speed 1:
    /// - The first train ride takes 1/1 = 1 hour.
    /// - Since we are already at an integer hour, we depart immediately at the 1 hour mark. The second train takes 3/1 = 3 hours.
    /// - Since we are already at an integer hour, we depart immediately at the 4 hour mark. The third train takes 2/1 = 2 hours.
    /// - You will arrive at exactly the 6 hour mark.
    ///
    /// Example 2:
    /// -----------
    /// - Input: dist = [1,3,2], hour = 2.7
    /// - Output: 3
    /// 
    /// Explanation: At speed 3:
    /// - The first train ride takes 1/3 = 0.33333 hours.
    /// - Since we are not at an integer hour, we wait until the 1 hour mark to depart. The second train ride takes 3/3 = 1 hour.
    /// - Since we are already at an integer hour, we depart immediately at the 2 hour mark. The third train takes 2/3 = 0.66667 hours.
    /// - You will arrive at the 2.66667 hour mark.
    ///
    /// Example 3:
    /// -----------
    /// - Input: dist = [1,3,2], hour = 1.9
    /// - Output: -1
    /// 
    /// Explanation: It is impossible because the earliest the third train can depart is at the 2 hour mark.
    ///
    /// Constraints:
    /// ----------- 
    /// n == dist.length
    /// 1 <= n <= 105
    /// 1 <= dist[i] <= 105
    /// 1 <= hour <= 109
    /// There will be at most two digits after the decimal point in hour.
    /// 
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        unimplemented!()
    }

    /// ## 1283. Find the Smallest Divisor Given a Threshold
    /// https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/description/
    ///
    /// Given an array of integers nums and an integer threshold, we will choose a positive integer divisor, divide 
    /// all the array by it, and sum the division's result. Find the smallest divisor such that the result mentioned 
    /// above is less than or equal to threshold.
    ///
    /// Each result of the division is rounded to the nearest integer greater than or equal to that element. 
    /// (For example: 7/3 = 3 and 10/2 = 5).
    ///
    /// The test cases are generated so that there will be an answer.
    ///
    /// Example 1:
    /// ----------
    /// - Input: nums = [1,2,5,9], threshold = 6
    /// - Output: 5
    ///
    /// Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1. 
    /// If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2). 
    ///
    /// Example 2:
    /// -----------
    /// - Input: nums = [44,22,33,11,1], threshold = 5
    /// - Output: 44
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 5 * 104
    /// 1 <= nums[i] <= 106
    /// nums.length <= threshold <= 106
    ///
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        unimplemented!()
    }

    // # 1231. Divide Chocolate
    // https://leetcode.com/problems/divide-chocolate/

    // You have one chocolate bar that consists of some chunks. Each chunk has its own sweetness given by the array sweetness.
    //
    // You want to share the chocolate with your k friends so you start cutting the chocolate bar into k + 1 pieces using k cuts, 
    // each piece consists of some consecutive chunks.
    //
    // Being generous, you will eat the piece with the minimum total sweetness and give the other pieces to your friends.
    //
    // Find the maximum total sweetness of the piece you can get by cutting the chocolate bar optimally.
    //
    //
    //
    // Example 1:
    //
    // Input: sweetness = [1,2,3,4,5,6,7,8,9], k = 5
    // Output: 6
    // Explanation: You can divide the chocolate to [1,2,3], [4,5], [6], [7], [8], [9]
    // Example 2:
    //
    // Input: sweetness = [5,6,7,8,9,1,2,3,4], k = 8
    // Output: 1
    // Explanation: There is only one way to cut the bar into 9 pieces.
    // Example 3:
    //
    // Input: sweetness = [1,2,2,1,2,2,1,2,2], k = 2
    // Output: 5
    // Explanation: You can divide the chocolate to [1,2,2], [1,2,2], [1,2,2]
    //
    //
    // Constraints:
    //
    // 0 <= k < sweetness.length <= 104
    // 1 <= sweetness[i] <= 105
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        unimplemented!()
    }

    /// ## 410. Split Array Largest Sum
    ///
    /// Given an integer array nums and an integer k, split nums into k non-empty subarrays such that the largest 
    /// sum of any subarray is minimized.
    ///
    /// Return the minimized largest sum of the split.
    ///
    /// A subarray is a contiguous part of the array.
    ///
    /// Example 1:
    /// -----------
    /// - Input: nums = [7,2,5,10,8], k = 2
    /// - Output: 18
    ///
    /// Explanation: There are four ways to split nums into two subarrays.
    /// The best way is to split it into [7,2,5] and [10,8], where the largest sum among the two subarrays is only 18.
    ///
    /// Example 2:
    /// -----------
    /// - Input: nums = [1,2,3,4,5], k = 2
    /// - Output: 9
    ///
    /// Explanation: There are four ways to split nums into two subarrays.
    /// The best way is to split it into [1,2,3] and [4,5], where the largest sum among the two subarrays is only 9.
    ///
    /// Constraints:
    /// -----------
    /// 1 <= nums.length <= 1000
    /// 0 <= nums[i] <= 106
    /// 1 <= k <= min(50, nums.length)
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        unimplemented!()
    }

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_queries() {
        let nums = vec![4,5,2,1]; let queries = vec![3,10,21];
        assert_eq!(Solution::answer_queries(nums, queries), vec![2,3,4]);

        let nums = vec![2,3,4,5]; let queries = vec![1];    
        assert_eq!(Solution::answer_queries(nums, queries), vec![0]);
    }

    #[test]
    fn test_min_eating_speed() {
        let piles = vec![3,6,7,11]; let hour = 8; let target = 4;
        assert_eq!(Solution::min_eating_speed(piles, hour), target);

        let piles = vec![30,11,23,4,20]; let hour = 5; let target = 30;
        assert_eq!(Solution::min_eating_speed(piles, hour), target);

        let piles = vec![30,11,23,4,20]; let hour = 6; let target = 30;
        assert_eq!(Solution::min_eating_speed(piles, hour), target);
    }

    #[test]
    fn test_min_speed_on_time() {
        
        let dist = vec![1,3,2]; let hour = 1.9; let target = 1;
        assert_eq!(Solution::min_speed_on_time(dist, hour), target);

        let dist = vec![1,3,2]; let hour = 2.7; let target = 3;
        assert_eq!(Solution::min_speed_on_time(dist, hour), target);

        let dist = vec![1,3,2]; let hour = 1.9; let target = -1;
        assert_eq!(Solution::min_speed_on_time(dist, hour), target);
    }

    #[test]
    fn test_smallest_divisor() {
        let nums = vec![1,2,5,9]; let threshold = 6; let target = 5;
        assert_eq!(Solution::smallest_divisor(nums, threshold), target);    

        let nums = vec![44,22,33,11,1]; let threshold = 5; let target = 44; 
        assert_eq!(Solution::smallest_divisor(nums, threshold), target);
    }

    #[test]
    fn test_split_array() {
        let nums = vec![7,2,5,10,8]; let k = 2; let target = 18;
        assert_eq!(Solution::split_array(nums, k), target);

        let nums = vec![1,2,3,4,5]; let k = 2; let target = 9;
        assert_eq!(Solution::split_array(nums, k), target);
    }

    #[test]
    fn test_maximize_sweetness() {
        let sweetness = vec![1,2,3,4,5,6,7,8,9]; let k = 5; let target = 6;
        assert_eq!(Solution::maximize_sweetness(sweetness, k), target);

        let sweetness = vec![5,6,7,8,9,1,2,3,4]; let k = 8; let target = 1;
        assert_eq!(Solution::maximize_sweetness(sweetness, k), target);

        let sweetness = vec![1,2,2,1,2,2,1,2,2]; let k = 2; let target = 5;
        assert_eq!(Solution::maximize_sweetness(sweetness, k), target);
    }

    #[test]
    fn test_search() {
        let nums = vec![-1,0,3,5,9,12]; let target = 9;
        assert_eq!(Solution::search(nums, target), 4);

        let nums = vec![-1,0,3,5,9,12]; let target = 2;
        assert_eq!(Solution::search(nums, target), -1);

        let nums = vec![5]; let target = 5;
        assert_eq!(Solution::search(nums, target), 0);
    }

    #[test]
    fn test_search_insert() {
        let nums = vec![1,3,5,6]; let target = 5;
        assert_eq!(Solution::search_insert(nums, target), 2);
        
        let nums = vec![1,3,5,6]; let target = 2;
        assert_eq!(Solution::search_insert(nums, target), 1);

        let nums = vec![1,3,5,6]; let target = 7;
        assert!(Solution::search_insert(nums, target) == 4);
    }
}