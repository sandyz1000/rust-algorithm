"""
Given k sorted arrays of size n each, merge them and print the sorted output.

Input:
k = 3, n =  4
arr = [[1, 3, 5, 7],
       [2, 4, 6, 8],
       [0, 9, 10, 11]]

Output:
0 1 2 3 4 5 6 7 8 9 10 11

A simple solution is to create an output array of size n*k and one by one copy 
all arrays to it.
Finally, sort the output array using any O(nLogn) sorting algorithm.

Time Complexity:
The main step is 3rd step, the loop runs n*k times. In every iteration of loop, we 
call heapify which takes O(Logk) time. Therefore, the time complexity is O(nk Logk).

"""

from typing import List
import heapq


def mergeK_sorted_array(arr: List[List[int]]) -> List[int]:
    """
    - Add the element and row, col in the heap
    - Check if col is not exhausted; then add the next element in heap
    - Time complexity: O(nk log k)
    :param _type_ arr: _description_
    :return _type_: _description_
    """
    n, k = len(arr), len(arr[0])
    # Add element in the heap in (element, (row, col))
    __heap = [(arr[i][0], (i, 0)) for i in range(n)]
    heapq.heapify(__heap)
    ans = []
    while __heap:
        top, (row, col) = heapq.heappop(__heap)
        if col + 1 < k:
            # Add the next element from the same row to heap
            heapq.heappush(__heap, (arr[row][col + 1], (row, col + 1)))
        ans.append(top)

    return ans


def cli_main():
    arr = [
        [1, 3, 5, 7],
        [2, 4, 6, 8],
        [0, 9, 10, 11]
    ]
    output = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
    res = mergeK_sorted_array(arr)
    print(res)
    assert res == output


if __name__ == "__main__":
    cli_main()
