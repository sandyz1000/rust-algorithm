"""
https://leetcode.com/problems/longest-increasing-subsequence/

Longest Increasing Subsequence
------------------------------
Given an integer array nums, return the length of the longest strictly increasing 
subsequence.

Example 1:

Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
Example 2:

Input: nums = [0,1,0,3,2,3]
Output: 4
Example 3:

Input: nums = [7,7,7,7,7,7,7]
Output: 1

Constraints:
------------
1 <= nums.length <= 2500
-104 <= nums[i] <= 104

Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
"""
import sys
from typing import List
from functools import lru_cache


def longest_increasing_recurse(nums: List) -> int:
    """
    Recurse with start index and prev value
    If current > prev; than add + 1 to ans and set prev = current and recurse
    If not than skip and compare with the next start index
    
    nums = [10, 9, 2, 5, 3, 7, 101, 18]
    lis = [2, 3, 7, 18]
    """
    @lru_cache
    def _lis(prev: int, start: int):
        if start >= len(nums):
            return 0
        taken = 0
        # Take the current pos
        if nums[start] > prev:
            taken = 1 + _lis(nums[start], start + 1)
        # Skip the current index
        not_taken = _lis(prev, start + 1)
        return max(taken, not_taken)
    
    return _lis(-sys.maxsize, 0)


def longest_increasing_recurseII(nums: List[int]) -> int:
    """
    Recurse with start index
    Iterate from start_idx + 1 and compute the max_length from the start + 1 pos
    Compare and check if nums[i] > start_idx; than ans = 1 + lis(start_idx + 1)
    Else start from next index
    return the max_length
    nums = [10, 9, 2, 5, 3, 7, 101, 18]
    lis = [2, 3, 7, 18]
    """
    # TODO: FixMe
    @lru_cache
    def __lis(start_idx: int):
        ans = 0
        for i in range(start_idx + 1, len(nums)):
            if nums[i] > nums[start_idx]:
                ans = max(ans, 1 + __lis(i))
            else:
                ans = max(ans, __lis(i + 1))

        return ans
    
    return __lis(0)


def lis_monoqueue(nums: List) -> int:
    # create mono stack and add in non-decreasing order

    stack, right = [], len(nums) - 1
    max_length = 0
    while right > 0:
        while stack and stack[-1] < nums[right]:
            max_length = max(max_length, len(stack))
            stack.pop()

        stack.append(nums[right])
        right -= 1

    return max(max_length, len(stack))


def main():
    nums = [10, 9, 2, 5, 3, 7, 101, 18]
    ans = longest_increasing_recurseII(nums)
    print(ans)
    assert ans == 4

    # assert longest_increasing_recurse([7, 7, 7, 7, 7, 7, 7]) == 1

    # x = longest_increasing_recurse([0, 1, 0, 3, 2, 3])
    # assert x == 4

    # z = longest_increasing_recurse([10, 9, 2, 5, 3, 7, 101, 18])
    # print(z)
    # assert z == 4


if __name__ == "__main__":
    main()
