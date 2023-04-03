"""
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

"""
from typing import List
import heapq
from dataclasses import dataclass, field


@dataclass(order=True)
class ListNode:
    val: int
    next: 'ListNode' = field(default=None, compare=False)


def mergeK_sorted(arr: List[ListNode]):
    """
    - Use heap to add k node in the heap
    - Get the min and add the value to dummy and move to next pointer 
    iff next is present
    - Time Complexity: O( n*k log K)

    :param List[ListNode] arr: _description_
    """
    __heap = [(node.val, node) for node in arr]
    dummy = ListNode(-1)
    head = dummy
    heapq.heapify(__heap)
    
    while __heap:
        value, top = heapq.heappop(__heap)

        if top.next:
            heapq.heappush(__heap, (top.next.val, top.next))
        head.next = ListNode(value)
        head = head.next
    
    return dummy.next


def init(arr: List):
    dummy = ListNode(-1, next=None)
    head = dummy
    for element in arr:
        dummy.next = ListNode(element)
        dummy = dummy.next

    return head.next


def main():
    llists = [[1, 4, 5], [1, 3, 4], [2, 6]]
    llists = [init(arr) for arr in llists]
    result = mergeK_sorted(llists)
    output = init([1, 1, 2, 3, 4, 4, 5, 6])
    assert result == output


if __name__ == "__main__":
    main()
