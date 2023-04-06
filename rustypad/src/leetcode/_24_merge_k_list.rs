/* 


Merge k Sorted Lists

You are given an array of k linked-lists lists, each linked-list is sorted in 
ascending order.

Merge all the linked-lists into one sorted linked-list and return it.


Example 1:

Input: lists = [[1,4,5],[1,3,4],[2,6]]
Output: [1,1,2,3,4,4,5,6]
Explanation: The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]
merging them into one sorted list:
1->1->2->3->4->4->5->6
Example 2:

Input: lists = []
Output: []
Example 3:

Input: lists = [[]]
Output: []


- Use heap to add k node in the heap
- Get the min and add the value to dummy and move to next pointer iff next is present
- Time Complexity: O( n*k log K)
 
*/