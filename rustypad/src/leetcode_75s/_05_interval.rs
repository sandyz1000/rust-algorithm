#![allow(unused)]
use std::cmp::Reverse;
use std::collections::BinaryHeap;


#[derive(Debug, Default)]
struct Solution;

impl Solution {
    
    /// ## Meeting Rooms II
    /// https://leetcode.com/problems/meeting-rooms-ii/description/
    ///
    ///
    /// Given an array of meeting time intervals intervals where intervals[i] = [starti, endi],
    /// return the minimum number of conference rooms required.
    ///
    /// Example 1:
    /// ----------
    /// - Input: intervals = [[0,30],[5,10],[15,20]]
    /// - Output: 2
    ///
    /// Example 2:
    /// ----------
    /// - Input: intervals = [[7,10],[2,4]]
    /// - Output: 1
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= intervals.length <= 104
    /// - 0 <= starti < endi <= 106
    ///
    /// Add endtime to heap; if start_time > heap[0]; remove the top element and decrease the count
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

    /// ## Merge Intervals
    /// https://leetcode.com/problems/merge-intervals/
    ///
    ///
    /// Given an array of intervals where intervals[i] = [starti, endi], merge all
    /// overlapping intervals, and return an array of the non-overlapping intervals
    /// that cover all the intervals in the input.
    ///
    /// Example 1:
    /// ----------
    /// - Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
    /// - Output: [[1,6],[8,10],[15,18]]
    /// - Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
    ///
    /// Example 2:
    /// ----------
    /// - Input: intervals = [[1,4],[4,5]]
    /// - Output: [[1,5]]
    /// - Explanation: Intervals [1,4] and [4,5] are considered overlapping.
    ///  
    /// Constraints:
    /// ----------
    /// 1 <= intervals.length <= 104
    /// intervals[i].length == 2
    /// 0 <= starti <= endi <= 104
    ///
    fn merge_interval(mut intervals: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
        let mut ans: Vec<[i32; 2]> = Vec::new();
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        ans.push(intervals[0]);
        // check if last end-time > current start_time; then merge into mergelist
        for item in intervals.iter().skip(1) {
            let start = item[0];
            let end = item[1];
            let last = ans.last().unwrap();
            if start <= last[1] {
                ans.push([last[0], end.max(last[1])]);
            } else {
                ans.push([start, end]);
            }
        }
        ans
    }

    /// ## 57. Insert Interval
    /// https://leetcode.com/problems/insert-interval/description/
    ///
    ///
    /// You are given an array of non-overlapping intervals intervals where
    /// intervals[i] = [start_i, end_i] represent the start and the end of the ith
    /// interval and intervals is sorted in ascending order by start_i.
    /// You are also given an interval newInterval = [start, end] that represents the
    /// start and end of another interval.
    ///
    /// Insert newInterval into intervals such that intervals is still sorted in ascending
    /// order by start_i and intervals still does not have any overlapping intervals (merge
    /// overlapping intervals if necessary).
    ///
    /// Return intervals after the insertion.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let intervals = vec![vec![1,3],vec![6,9]]; let newInterval = vec![2,5];
    /// let ans = vec![vec![1,5],vec![6,9]];
    /// assert_eq!(Solution::insert(intervals, newInterval), ans);
    /// ```
    /// Example 2:
    /// ----------
    /// ```
    /// let intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]]; let newInterval = vec![4,8];
    /// let ans = vec![vec![1,2],vec![3,10],vec![12,16]];
    /// assert_eq!(Solution::insert(intervals, newInterval), ans);
    /// ```
    /// - Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
    ///
    /// Constraints:
    /// ----------
    /// - 0 <= intervals.length <= 104
    /// - intervals[i].length == 2
    /// - 0 <= starti <= endi <= 105
    ///
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        
        fn insert_interval(intervals: &mut Vec<Vec<i32>>, new_interval: Vec<i32>) {
            let mut inserted = false;
            for i in 0..intervals.len() {
                if intervals[i][0] > new_interval[0] {
                    intervals.insert(i, new_interval.clone());
                    inserted = true;
                    break;
                }
            }
            if !inserted {
                intervals.push(new_interval.clone());
            }
        }
        
        // This will insert new interval into intervals at specified index based on start date
        insert_interval(&mut intervals, new_interval);

        let mut ans: Vec<Vec<i32>> = vec![intervals[0].clone()];
        for i in 1..intervals.len() {
            let interval = intervals[i].clone();
            let (mut start, mut end) = (interval[0], interval[1]);
            // Here merge the interval if any overlap found
            
            // If the last end is smaller than current start
            if ans.last().unwrap()[1] < start {
                ans.push(interval);
            } else {
                let last = ans.pop().unwrap();
                ans.push(vec![last[0], end.max(last[1])]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_no_of_meeting_rooms() {
        // Test 1
        let intervals = vec![[0, 30], [5, 10], [15, 20]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 2);
        println!(">>>> Test-1 result >>>> {}", result);

        // Test 2
        let intervals = vec![[7, 10], [2, 4]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 1);
        println!(">>>> Test-2 result >>>> {}", result);

        // Test 3
        let intervals = vec![[13, 15], [1, 13]];
        let result = Solution::no_of_meeting_rooms(intervals);
        assert_eq!(result, 1);
        println!(">>>> Test-3 result >>>> {}", result);
    }

    #[test]
    fn test_merge_interval() {
        let intervals = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
        assert_eq!(
            Solution::merge_interval(intervals),
            vec![[1, 6], [8, 10], [15, 18]]
        );
    }

    #[test]
    fn test_insert_interval() {

    }
}
