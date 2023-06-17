/* 

4. Median of Two Sorted Arrays
https://leetcode.com/problems/median-of-two-sorted-arrays/description/

Given two sorted arrays nums1 and nums2 of size m and n respectively, return the 
median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).

Example 1:
----------
Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.

Example 2:
----------
Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Constraints:
------------
nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000

-106 <= nums1[i], nums2[i] <= 106

 */

/// - The function find_median_sorted_arrays takes two vectors as input and returns the median 
/// of their sorted values as a f64 value. 
/// 
/// - The function first checks the length of the two vectors and makes sure that nums1 is not 
/// larger than nums2 by swapping them if necessary. 
/// 
/// - The function then performs a binary search on the indices of nums1 to find the partition 
/// such that the left half of nums1 and nums2 together form half of the total number of elements. 
/// 
/// - The function then checks if the partition is correct by comparing the maximum of the left side 
/// of nums1 and nums2 with the minimum of the right side of nums1 and nums2. 
/// 
/// - If the partition is correct, the function returns the median of the two vectors by taking the 
/// average of the maximum of the left side and the minimum of the right side if the total number 
/// of elements is even, or simply the maximum of the left side if the total number of elements is odd. 
/// 
/// - If the partition is not correct, the function updates the search range and repeats the binary search. 
/// 
/// - If the function cannot find a valid partition, it panics.
#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (m, n) = (nums1.len(), nums2.len());
    if m > n {
        return find_median_sorted_arrays(nums2, nums1);
    }
    let (mut left, mut right) = (0, m);
    while left <= right {
        let i = (left + right) / 2;
        let j = (m + n + 1) / 2 - i;
        let max_left_nums1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
        let min_right_nums1 = if i == m { i32::MAX } else { nums1[i] };
        let max_left_nums2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
        let min_right_nums2 = if j == n { i32::MAX } else { nums2[j] };
        
        if max_left_nums1 <= min_right_nums2 && max_left_nums2 <= min_right_nums1 {
            if (m + n) % 2 == 0 {
                return (f64::from(max_left_nums1.max(max_left_nums2))
                        + f64::from(min_right_nums1.min(min_right_nums2))) / 2.0;
            } else {
                return f64::from(max_left_nums1.max(max_left_nums2));
            }
        } else if max_left_nums1 > min_right_nums2 {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }
    panic!("Solution should always exist.");
}

#[allow(dead_code)]
fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
}

mod tests {
    use super::*;
    
    /// Test case 1
    #[test]
    fn test_find_median_sorted_arrays_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let expected_output = 2.0;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }    

    #[test]
    fn test_find_median_sorted_arrays_2() {
        // Test case 2
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let expected_output = 2.5;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }

    #[test]
    fn test_find_median_sorted_arrays_3() {
        // Test case 3
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6];
        let expected_output = 3.5;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }

    #[test]
    fn test_find_median_sorted_arrays_4() {
        // Test case 4
        let nums1 = vec![1, 2, 3, 4];
        let nums2 = vec![5, 6, 7, 8];
        let expected_output = 4.5;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }

    #[test]
    fn test_find_median_sorted_arrays_5() {
        // Test case 5
        let nums1 = vec![1];
        let nums2 = vec![2, 3, 4];
        let expected_output = 2.5;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }

    #[test]
    fn test_find_median_sorted_arrays_6() {
        // Test case 6
        let nums1 = vec![];
        let nums2 = vec![1];
        let expected_output = 1.0;
        assert_eq!(find_median_sorted_arrays(nums1, nums2), expected_output);
    }
}
