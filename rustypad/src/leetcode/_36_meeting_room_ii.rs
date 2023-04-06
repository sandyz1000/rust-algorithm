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

impl Solution {
    fn no_of_meeting_rooms(mut intervals: Vec<[i32; 2]>) -> i32 {
        let meeting_room = 0;
        // Sort with start time
        intervals.sort();
        // intervals = sorted(intervals, key=lambda x: x[0])  // nlogn
        // Min-heap to add endtime
        let heapq: BinaryHeap<i32> = BinaryHeap::new();
        for (start, end) in intervals.iter().enumerate() {
            if !heapq.is_empty() && start >= heapq.peek().unwrap() {
                // If start at the same time as other meeting end
                heapq.pop();
                meeting_room -= 1;
            }
            heapq.push(end[0]);
            meeting_room += 1;
        }
        // for start, end in intervals:
        //     # 
        //     if end_time and start >= end_time[0]:
        //         heapq.heappop(end_time)
        //         meeting_room -= 1
        //     heapq.heappush(end_time, end)
        //     meeting_room += 1

        meeting_room
    }

}

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