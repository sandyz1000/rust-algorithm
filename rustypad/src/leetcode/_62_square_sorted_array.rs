/* 
977. Squares of a Sorted Array

Given an integer array nums sorted in non-decreasing order, return an array of the 
squares of each number sorted in non-decreasing order.

Example 1:
----------
Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].

Example 2:
----------
Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]

 */
#![allow(unused)]

struct Solution;

impl Solution {
    fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        // Two pointer with start and end
        // Start filling the result array from the end by comparing start & end
        // Reverse the result and return
        let capacity: i32 = nums.len() as i32;
        let mut ans: Vec<i32> = vec![];
        let (mut start, mut end): (i32, i32) = (0, capacity - 1);
        let mut ans_idx: i32 = capacity- 1;
        while start <= end {
            let left: i32 = nums[start as usize].pow(2);
            let right: i32 = nums[end as usize].pow(2);
            if left < right {
                ans.push(right);
                end -= 1;
            } else {
                ans.push(left);
                start += 1;
            }
            ans_idx -= 1;
        }
        
        ans.reverse();
        ans
    }

}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test1() {
        let nums = vec![-4,-1,0,3,10];
        let ans = vec![0,1,9,16,100];
        assert_eq!(Solution::sorted_squares(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![-7,-3,2,3,11];
        let ans = vec![4,9,9,49,121];
        assert_eq!(Solution::sorted_squares(nums), ans);
    }
}