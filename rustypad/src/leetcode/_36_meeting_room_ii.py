"""
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

"""
# Add endtime to heap; if start_time > heap[0]; remove the top element and decrease the count
import heapq
from typing import List


def no_of_meetingRooms(intervals: List[List[int]]) -> int:
    meeting_room = 0
    # Sort with start time
    intervals = sorted(intervals, key=lambda x: x[0])  # nlogn
    # Min-heap to add endtime
    end_time = []
    for start, end in intervals:
        # If start at the same time as other meeting end
        if end_time and start >= end_time[0]:
            heapq.heappop(end_time)
            meeting_room -= 1
        heapq.heappush(end_time, end)
        meeting_room += 1

    return meeting_room


def main():
    intervals = [[0, 30], [5, 10], [15, 20]]
    result = no_of_meetingRooms(intervals)
    assert result == 2
    print(">>>> Test-1 result >>>>", result)

    intervals = [[7, 10], [2, 4]]
    result = no_of_meetingRooms(intervals)
    assert result == 1
    print(">>>> Test-2 result >>>>", result)

    intervals = [[13, 15], [1, 13]]
    result = no_of_meetingRooms(intervals)
    assert result == 1
    print(">>>> Test-3 result >>>>", result)


if __name__ == "__main__":
    main()
