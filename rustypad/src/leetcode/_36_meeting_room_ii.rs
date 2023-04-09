/* 
https://leetcode.com/problems/meeting-rooms-ii/description/

Meeting Rooms II

Given an array of meeting time intervals intervals where intervals[i] = [starti, endi], 
return the minimum number of conference rooms required.

Example 1:
----------
Input: intervals = [[0,30],[5,10],[15,20]]
Output: 2

Example 2:
----------
Input: intervals = [[7,10],[2,4]]
Output: 1

Constraints:
------------
1 <= intervals.length <= 104
0 <= starti < endi <= 106

Add endtime to heap; if start_time > heap[0]; remove the top element and decrease the count


 */

#[derive(Debug, Default)]
struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;


impl Solution {
    fn no_of_meeting_rooms(mut intervals: Vec<[i32; 2]>) -> i32 {
        let mut meeting_room = 0;
        // Sort with start time
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        
        // Min-heap to add endtime
        let mut heapq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        
        for interval in intervals.iter() {
            let (start, end) = (interval[0], interval[1]);
            if let Some(Reverse(top)) = heapq.peek() {
                if !heapq.is_empty() && start >= *top {
                    // If start at the same time as other meeting end
                    heapq.pop();
                    meeting_room -= 1;
                }
            }
            heapq.push(Reverse(end));
            meeting_room += 1;
        }
        meeting_room
    }

}

#[cfg(test)]
mod tests {
    use super::Solution;
    
    #[test]
    fn test_one() {
        let intervals = vec![[0, 30], [5, 10], [15, 20]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 2);
        println!(">>>> Test-1 result >>>> {}", result);
    
    }
    
    #[test]
    fn test_two() {
        let intervals = vec![[7, 10], [2, 4]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 1);
        println!(">>>> Test-2 result >>>> {}", result);
    }
    
    #[test]
    fn test_three() {
        let intervals = vec![[13, 15], [1, 13]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 1);
        println!(">>>> Test-3 result >>>> {}", result);
    }
}
