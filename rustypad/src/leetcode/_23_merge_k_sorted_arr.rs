/* 
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

- Add the element and row, col in the heap
- Check if col is not exhausted; then add the next element in heap
- Time complexity: O(nk log k)

 */
