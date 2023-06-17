#![allow(unused)]

use core::num;
use std::{collections::BinaryHeap, fmt::Binary, cmp::Reverse, ops::Deref};

/// ## 703. Kth Largest Element in a Stream
/// 
/// https://leetcode.com/problems/kth-largest-element-in-a-stream/description/
///
/// Design a class to find the kth largest element in a stream. Note that it is the kth largest element in 
/// the sorted order, not the kth distinct element.
///
/// Implement KthLargest class:
///
/// KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
/// int add(int val) Appends the integer val to the stream and returns the element representing the kth 
/// largest element in the stream.
///  
///
/// Example 1:
/// ----------
/// Input
/// ["KthLargest", "add", "add", "add", "add", "add"]
/// [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
/// Output
/// [null, 4, 5, 5, 8, 8]
///
/// Explanation
/// KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
/// kthLargest.add(3);   // return 4
/// kthLargest.add(5);   // return 5
/// kthLargest.add(10);  // return 5
/// kthLargest.add(9);   // return 8
/// kthLargest.add(4);   // return 8
///  
///
/// Constraints:
/// -----------
/// 1 <= k <= 104
/// 0 <= nums.length <= 104
/// -104 <= nums[i] <= 104
/// -104 <= val <= 104
/// At most 104 calls will be made to add.
/// It is guaranteed that there will be at least k elements in the array when you search for the kth element.
struct KthLargest {
    k: usize,
    hq: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        
        // Use min-heap to keep track of the k largest elements
        let mut hq: BinaryHeap<Reverse<i32>> = nums.into_iter().map(|x| Reverse(x)).collect();
        while hq.len() > k {
            hq.pop();
        }
        Self { k, hq }
    }
    
    fn add(&mut self, val: i32) -> i32 {
        self.hq.push(Reverse(val));
        if self.hq.len() > self.k {
            self.hq.pop();
        }
        let ans = self.hq.peek().unwrap();
        ans.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        let ret_1: i32 = obj.add(3);
        assert_eq!(ret_1, 4);

        let ret_2: i32 = obj.add(5);
        assert_eq!(ret_2, 5);

        let ret_3: i32 = obj.add(10);
        assert_eq!(ret_3, 5);

        let ret_4: i32 = obj.add(9);
        assert_eq!(ret_4, 8);

        let ret_5: i32 = obj.add(4);
        assert_eq!(ret_5, 8);

    }
}
