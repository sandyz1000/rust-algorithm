"""
https://leetcode.com/problems/sort-list/
Given the head of a linked list, return the list after sorting it in ascending order.    

Input: head = [4,2,1,3]
Output: [1,2,3,4]

Input: head = [-1,5,3,4,0]
Output: [-1,0,3,4,5]

Input: head = []
Output: []
"""
from dataclasses import dataclass
from typing import List


@dataclass
class ListNode:
    val: int
    next: 'ListNode'


def sortList(head: ListNode) -> ListNode:
    """
    Merge sort
    - Find mid
    - recurse to sortList for left and right 
    - merge sort list

    :param _type_ head: _description_
    :return _type_: _description_
    """
    if head is None or head.next is None:
        return head
    
    mid = get_mid(head)
    left = sortList(head)
    right = sortList(mid)
    return merge(left, right)


def merge(head1: ListNode, head2: ListNode) -> ListNode:
    """
    - create dummy node
    - compare and copy smaller element to dummy
    """
    dummy = ListNode(-1)
    curr = dummy
    while head1 and head2:
        if head1.val < head2.val:
            curr.next = ListNode(head1.val)
            head1 = head1.next
        else:
            curr.next = ListNode(head2.val)
            head2 = head2.next
        curr = curr.next
    
    # Copy rest of the element to dummy
    while head1:
        curr.next = ListNode(head1.val)
        head1 = head1.next
        curr = curr.next

    while head2:
        curr.next = ListNode(head2.val)
        head2 = head2.next
        curr = curr.next

    return dummy.next
     

def get_mid(head: ListNode) -> ListNode:
    """
    1 -> 2 -> 3 -> 4
    - Two pointer fast and mid
    - 
    :param _type_ node: _description_
    """
    midPrev = head
    while (head and head.next):
        midPrev = midPrev.next
        head = head.next.next
    mid, midPrev.next = midPrev.next, None
    return mid


def init(arr: List):
    dummy = ListNode(-1, next=None)
    head = dummy
    for element in arr:
        dummy.next = ListNode(element)
        dummy = dummy.next

    return head.next


def main():
    arr = [4, 2, 1, 3]
    head1 = init(arr)
    output = sortList(head1)
    assert output == init(sorted(arr))

    arr = [-1, 5, 3, 4, 0]
    head2 = init(arr)
    output = sortList(head2)
    assert output == init([-1, 0, 3, 4, 5])


if __name__ == "__main__":
    main()
