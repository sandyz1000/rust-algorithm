
#![allow(unused)]

/// ## 912. Sort an Array
/// https://leetcode.com/problems/sort-an-array/description/
///
/// Given an array of integers nums, sort the array in ascending order and return it.
///
/// You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the 
/// smallest space complexity possible.
///
///  
///
/// Example 1:
/// ----------
/// ```
/// let nums = vec![5,2,3,1];
/// assert_eq!(Solution::sort_array(nums), vec![1,2,3,5]);
/// ```
/// *Explanation*: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), 
/// while the positions of other numbers are changed (for example, 1 and 5).
///
// Example 2:
/// ---------
/// ```
/// let nums = vec![5,1,1,2,0,0];
/// assert_eq!(Solution::sort_array(nums), vec![0,0,1,1,2,5]);
/// ```
/// *Explanation*: Note that the values of nums are not necessairly unique.
///
/// Constraints:
/// ------------
/// * 1 <= nums.length <= 5 * 104
/// * -5 * 104 <= nums[i] <= 5 * 104
struct Solution;


impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn quick_sort(nums: &mut Vec<i32>, low: i32, high: i32) {
            let mut i = low;
            let mut j = high;
            // Find pivot
            let pivot = low + (high - low) / 2;
            while i <= j {
                while nums[i as usize] < nums[pivot as usize] {
                    i +=1;
                }

                while nums[j as usize] > nums[pivot as usize] {
                    j -=1;
                }

                if i <= j {
                    // Swap
                    nums.swap(i as usize, j as usize);
                    i +=1;
                    j -=1;
                }
            }
            if low < j {
                quick_sort(nums, low, j);
            }
            if i < high {
                quick_sort(nums, i, high);
            }
        }

        let length = nums.len() as i32;
        quick_sort(&mut nums, 0,  length - 1);
        nums
    }
}

// TODO: Fix tests
#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5,2,3,1];
        assert_eq!(Solution::sort_array(nums), vec![1,2,3,5]);
    }

    #[test]
    fn test2() {        
        let nums = vec![5,1,1,2,0,0];
        assert_eq!(Solution::sort_array(nums), vec![0,0,1,1,2,5]);
    }

    #[test]
    fn test3() {
        let nums = vec![-1,2,-8,-10];
        assert_eq!(Solution::sort_array(nums), vec![-10,-8,-1,2]);
    }
}