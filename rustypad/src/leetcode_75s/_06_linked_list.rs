
#![allow(unused)]

use std::cmp::{Reverse, Ordering};
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }
}
type ListLink = Option<Box<ListNode>>;

trait ListMaker {
    fn new(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode {val, next} ))
    }
}

impl ListMaker for ListLink {}

#[macro_use]
macro_rules! linked_list {
    () => {
        None
    };
    ($t:expr) => {
        ListLink::new($t, None)
    };
    ($t:expr, $($tail:tt)*) => {
        ListLink::new($t, linked_list!(
            $( $tail )* 
        ))
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Item {
    priority: i32,
    node: Box<ListNode>,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.priority.cmp(&other.priority))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

struct Solution;

impl Solution {
    /// ## 23. Merge k Sorted Lists
    /// https://leetcode.com/problems/merge-k-sorted-lists/
    /// 
    /// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
    ///
    /// Merge all the linked-lists into one sorted linked-list and return it.
    ///
    /// Example 1:
    /// ----------
    /// - Input: lists = [[1,4,5],[1,3,4],[2,6]]
    /// - Output: [1,1,2,3,4,4,5,6]
    /// - Explanation: The linked-lists are:
    /// [
    /// 1->4->5,
    /// 1->3->4,
    /// 2->6
    /// ]
    /// merging them into one sorted list:
    /// 1->1->2->3->4->4->5->6
    /// 
    /// Example 2:
    /// ----------
    /// - Input: lists = []
    /// - Output: []
    /// 
    /// Example 3:
    /// ----------
    /// Input: lists = [[]]
    /// Output: []
    ///
    /// Constraints:
    /// ----------- 
    /// k == lists.length
    /// 0 <= k <= 104
    /// 0 <= lists[i].length <= 500
    /// -104 <= lists[i][j] <= 104
    /// lists[i] is sorted in ascending order.
    /// The sum of lists[i].length will not exceed 104.
    ///
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<Item>> = BinaryHeap::new();
        for node in lists {
            if let Some(node) = node {
                let item = Item { priority: node.val, node };
                heap.push(Reverse(item));
            }
        }
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;
        while let Some(Reverse(item)) = heap.pop() {
            curr.next = ListLink::new(item.priority, None);
            curr = curr.next.as_mut().unwrap();
            if let Some(next) = item.node.next {
                let next_item = Item { priority: next.val, node: next };
                heap.push(Reverse(next_item));
            }
        }
        dummy.next
    }

    /// ## 19. Remove Nth Node From End of List
    /// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
    /// 
    /// Given the head of a linked list, remove the nth node from the end of the list and return its head.
    ///
    /// Example 1:
    /// ----------
    /// Input: head = [1,2,3,4,5], n = 2
    /// Output: [1,2,3,5]
    ///
    /// Example 2:
    /// ----------
    /// Input: head = [1], n = 1
    /// Output: []
    ///
    /// Example 3:
    /// ----------
    /// Input: head = [1,2], n = 1
    /// Output: [1]
    ///
    /// Constraints:
    /// ----------
    /// The number of nodes in the list is sz.
    /// 1 <= sz <= 30
    /// 0 <= Node.val <= 100
    /// 1 <= n <= sz
    pub fn remove_nth_from_end(head: ListLink, n: i32) -> ListLink {
        let mut dummy = Box::new(ListNode {val: -1, next: head});
        
        // Scan to get the length of the list
        let mut curr = dummy.next.as_ref();
        let mut len = 0;
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }

        // Iterate for (len - n)th node
        let mut first = Some(&mut dummy);
        for _ in 0..(len - n) { 
            first = first?.next.as_mut();
        }
        
        // Connect the N-n-1 next node to next.next 

        if let Some(curr) = first {
            curr.next = curr.next.as_mut()?.next.take();
        }
        
        dummy.next
        
    }

    /// ## 143. Reorder List
    /// https://leetcode.com/problems/reorder-list/description/
    ///
    /// You are given the head of a singly linked-list. The list can be represented as:
    ///
    /// L0 → L1 → … → Ln - 1 → Ln
    /// Reorder the list to be on the following form:
    ///
    /// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
    /// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
    ///
    /// Example 1:
    /// ---------
    /// - Input: head = [1,2,3,4]
    /// - Output: [1,4,2,3]
    ///
    ///
    /// Example 2:
    /// ----------
    /// - Input: head = [1,2,3,4,5]
    /// - Output: [1,5,2,4,3]
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [1, 5 * 104].
    /// 1 <= Node.val <= 1000
    ///
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        unimplemented!()
    }


}


mod tests {
    use super::*;

    #[test]
    fn test_merge_k_lists() {
        let lists = vec![
            linked_list!(1,4,5), 
            linked_list!(1,3,4), 
            linked_list!(2,6)
        ];
        let res = Solution::merge_k_lists(lists);
        assert_eq!(res, linked_list!(1,1,2,3,4,4,5,6));

    }

    #[test]
    fn test_remove_nth_from_end() {
        fn test_one() {
            let head = linked_list!(1, 2, 3, 4, 5);
            assert_eq!(Solution::remove_nth_from_end(head, 2), linked_list!(1, 2, 3, 5));
        }

        fn test_two() {
            let head = linked_list!(1);
            assert_eq!(Solution::remove_nth_from_end(head, 1), None);
        }

        fn test_three() {
            let head = linked_list!(1, 2);
            assert_eq!(Solution::remove_nth_from_end(head, 1), linked_list!(1));
        }

        test_one();
        test_two();
        test_three();
    }
}