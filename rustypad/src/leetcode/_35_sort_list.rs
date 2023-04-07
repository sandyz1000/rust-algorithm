
/* 
https://leetcode.com/problems/sort-list/
Given the head of a linked list, return the list after sorting it in ascending order.    

Input: head = [4,2,1,3]
Output: [1,2,3,4]

Input: head = [-1,5,3,4,0]
Output: [-1,0,3,4,5]

Input: head = []
Output: []

     def init(arr: List):
    dummy = ListNode(-1, next=None)
    head = dummy
    for element in arr:
        dummy.next = ListNode(element)
        dummy = dummy.next

    return head.next



if __name__ == "__main__":
    main()


 */

struct Solution;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

impl Solution {

    fn new(arr: Vec<i32>) -> ListNode {
        todo!()
    }


    /// 1 -> 2 -> 3 -> 4
    /// Two pointer fast and mid
    fn get_mid(head: ListNode) -> ListNode {
        todo!()
        /* 
        midPrev = head
        while (head and head.next):
            midPrev = midPrev.next
            head = head.next.next
        mid, midPrev.next = midPrev.next, None
        return mid 
        */
    }
    
    /// - create dummy node
    /// - compare and copy smaller element to dummy
    fn merge(head1: ListNode, head2: ListNode) -> ListNode {
        todo!()
        /* 
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
        */
    }

    /// Merge sort
    /// - Find mid
    /// - recurse to sortList for left and right 
    /// - merge sort list
    fn sort_list(head: ListNode) -> ListNode {
        // if head is None or head.next is None:
        //     return head
        
        // mid = get_mid(head)
        // left = sortList(head)
        // right = sortList(mid)
        // return merge(left, right)
        todo!()
    }

}

#[test]
fn test1() {
    let arr = vec![4, 2, 1, 3];
    let head1: ListNode = Solution::new(arr);
    let output = Solution::sort_list(head1);
    // assert!(output, init(sorted(arr)))
}

#[test]
fn test2() {
    let arr = vec![-1, 5, 3, 4, 0];
    let head2: ListNode = Solution::new(arr);
    let output = Solution::sort_list(head2);
    // assert output == init([-1, 0, 3, 4, 5])
}