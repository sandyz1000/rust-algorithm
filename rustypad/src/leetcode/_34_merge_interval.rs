/* 

https://leetcode.com/problems/merge-intervals/

Merge Intervals

Given an array of intervals where intervals[i] = [starti, endi], merge all 
overlapping intervals, and return an array of the non-overlapping intervals 
that cover all the intervals in the input.

Example 1:
----------
Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

Example 2:
----------
Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
 

Constraints:
----------
1 <= intervals.length <= 104
intervals[i].length == 2
0 <= starti <= endi <= 104

 */


struct Solution;


impl Solution {

    #[allow(dead_code)]
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
            }
            else {
                ans.push([start, end]);
            }   
        }
        ans
    }
}

#[test]
fn test() {
    let intervals = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
    assert_eq!(Solution::merge_interval(intervals), vec![[1, 6], [8, 10], [15, 18]]);

}