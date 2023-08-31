#![allow(unused)]

/// ## 21. Merge Two Sorted Lists
///
/// https://leetcode.com/problems/merge-two-sorted-lists/description/
///
/// You are given the heads of two sorted linked lists list1 and list2.
///
/// Merge the two lists in a one sorted list. The list should be made by splicing
/// together the nodes of the first two lists.
///
/// Example 1:
/// ----------
/// ```
/// let list1 = linked_list![1,2,4]; list2 = linked_list![1,3,4];
/// assert_eq!(Solution::merge_two_lists(list1, list2), linked_list![1,1,2,3,4,4]);
/// ```
/// 
/// Example 2:
/// ----------
/// ```
/// let list1 = linked_list![]; list2 = linked_list![];
/// assert_eq!(Solution::merge_two_lists(list1, list2), linked_list![]);
/// ```
/// Example 3:
/// ----------
/// ```
/// let list1 = linked_list![]; let list2 = linked_list![0];
/// assert_eq!(Solution::merge_two_lists(list1, list2), linked_list![0]);
/// ```
struct Solution;

type ListLink = Option<Box<ListNode>>;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    #[allow(dead_code)]
    fn merge_two_lists(list1: ListLink, list2: ListLink) -> ListLink {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });

        // Iterate both list and compare the value
        let mut p1 = list1.as_ref();
        let mut p2 = list2.as_ref();
        let mut p3 = &mut dummy;
        while p1.is_some() && p2.is_some() {
            // When both the node value is present, add the value to p3
            if p1.is_some() && p2.is_some() {
                if p2?.val > p1?.val {
                    p3.next = Some(Box::new(ListNode {
                        val: p1.unwrap().val,
                        next: None,
                    }));
                    p1 = p1?.next.as_ref();
                } else {
                    p3.next = Some(Box::new(ListNode {
                        val: p2.unwrap().val,
                        next: None,
                    }));
                    p2 = p2?.next.as_ref();
                }
            }
            p3 = p3.next.as_mut().unwrap();
        }

        // When one of the node value is present, add the value to p3
        if p1.is_some() && p2.is_none() {
            p3.next = Some(p1?.clone());
        } else if p2.is_some() && p1.is_none() {
            p3.next = Some(p2?.clone());
        }

        dummy.next
    }
}

#[test]
fn test() {
    // let a = Solution::new(vec![1, 2, 4]);
    // let b = Solution::new(vec![1, 3, 4]);
    // let c = Solution::new(vec![1, 1, 2, 3, 4, 4]);
    // assert_eq!(Solution::merge_two_lists(a, b), c);
}
