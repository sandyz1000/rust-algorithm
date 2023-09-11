#![allow(unused)]

/// ## 295. Find Median from Data Stream
/// https://leetcode.com/problems/find-median-from-data-stream/
///
/// The median is the middle value in an ordered integer list. If the size of the list is even, there is no 
/// middle value, and the median is the mean of the two middle values.
///
/// For example, for arr = [2,3,4], the median is 3.
/// For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
/// Implement the MedianFinder class:
///
/// MedianFinder() initializes the MedianFinder object.
/// void addNum(int num) adds the integer num from the data stream to the data structure.
/// double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
///  
///
/// Example 1:
/// -----------
/// ```doc
/// Input
/// ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
/// [[], [1], [2], [], [3], []]
/// Output
/// [null, null, null, 1.5, null, 2.0]
/// ```
/// 
/// Explanation
/// ------------
/// ```doc
/// MedianFinder medianFinder = new MedianFinder();
/// medianFinder.addNum(1);    // arr = [1]
/// medianFinder.addNum(2);    // arr = [1, 2]
/// medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
/// medianFinder.addNum(3);    // arr[1, 2, 3]
/// medianFinder.findMedian(); // return 2.0
/// ```
/// ``` 
///
/// Constraints:
/// ------------
/// * -105 <= num <= 105
/// * There will be at least one element in the data structure before calling findMedian.
/// * At most 5 * 104 calls will be made to addNum and findMedian.
///  
///
/// Follow up:
/// -----------
/// * If all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
/// * If 99% of all integer numbers from the stream are in the range [0, 100], how would you optimize your solution?
use std::{collections::BinaryHeap, cmp::Reverse};

struct MedianFinder {
    min_heap: BinaryHeap<Reverse<i32>>,
    max_heap: BinaryHeap<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    
    /// Your MedianFinder object will be instantiated and called as such:
    /// ```
    /// let obj = MedianFinder::new();
    /// obj.add_num(num);
    /// let ret_2: f64 = obj.find_median();
    /// ```
    fn new() -> Self {
        Self {
            min_heap: BinaryHeap::new(),
            max_heap: BinaryHeap::new()
        }
    }
    
    fn add_num(&mut self, num: i32) {
        // Maintain the element in two heaps with the same size
        // Where the size of the left heap is equal to the size of the right heap in case of even

        // First add to min heap
        // Then add to max heap
        // If the min heap size is greater than the max heap size, then we need to pop the min heap

        self.max_heap.push(num);
        if let Some(item) = self.max_heap.pop() {
            self.min_heap.push(Reverse(item));
        }

        // If the second half len is greater than the first half len
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }
    
    fn find_median(&self) -> f64 {
        if (self.max_heap.len() + self.min_heap.len()) % 2 == 1 {
            *self.max_heap.peek().unwrap() as f64 
        } else {   
            (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0   
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        
    }
}