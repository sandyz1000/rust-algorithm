#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

type LinkList = Option<Rc<RefCell<ListNode>>>;

impl Solution {
    /// ## Intersection of Two Linked Lists
    /// https://leetcode.com/problems/intersection-of-two-linked-lists/description/
    ///
    /// Given the heads of two singly linked-lists headA and headB, return the node at which the two lists intersect. 
    /// If the two linked lists have no intersection at all, return null.
    ///
    /// For example, the following two linked lists begin to intersect at node c1:
    ///
    /// The test cases are generated such that there are no cycles anywhere in the entire linked structure.
    ///
    /// Note that the linked lists must retain their original structure after the function returns.
    ///
    /// Custom Judge:
    /// ============
    /// The inputs to the judge are given as follows (your program is not given these inputs):
    ///
    /// intersectVal - The value of the node where the intersection occurs. This is 0 if there is no intersected node.
    /// * listA - The first linked list.
    /// * listB - The second linked list.
    /// * skipA - The number of nodes to skip ahead in listA (starting from the head) to get to the intersected node.
    /// * skipB - The number of nodes to skip ahead in listB (starting from the head) to get to the intersected node.
    ///
    /// The judge will then create the linked structure based on these inputs and pass the two heads, headA and headB 
    /// to your program. If you correctly return the intersected node, then your solution will be accepted.
    ///
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let intersectVal = 8, let listA = vec![4,1,8,4,5]; let listB = vec![5,6,1,8,4,5]; let skipA = 2; skipB = 3;
    /// assert_eq!(Solution::intersect(listA, listB), vec![4,5]);
    /// // Output: Intersected at '8'
    /// ```
    /// Explanation: The intersected node's value is 8 (note that this must not be 0 if the two lists intersect).
    /// From the head of A, it reads as [4,1,8,4,5]. From the head of B, it reads as [5,6,1,8,4,5]. There are 2 nodes 
    /// before the intersected node in A; There are 3 nodes before the intersected node in B.
    /// - Note that the intersected node's value is not 1 because the nodes with value 1 in A and B (2nd node in A 
    /// and 3rd node in B) are different node references. In other words, they point to two different locations in memory, 
    /// while the nodes with value 8 in A and B (3rd node in A and 4th node in B) point to the same location in memory.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let intersectVal = 2; let listA = vec![1,9,1,2,4]; let listB = vec![3,2,4]; let skipA = 3; let skipB = 1;
    /// assert_eq!(Solution::intersect(listA, listB), vec![1,9]);
    /// // Output: Intersected at '2'
    /// ```
    /// Explanation: The intersected node's value is 2 (note that this must not be 0 if the two lists intersect).
    /// From the head of A, it reads as [1,9,1,2,4]. From the head of B, it reads as [3,2,4]. There are 3 nodes before 
    /// the intersected node in A; There are 1 node before the intersected node in B.
    ///
    /// Example 3:
    /// ---------
    /// ```
    /// let intersectVal = 0; let listA = vec![2,6,4]; let listB = vec![1,5]; let skipA = 3; let skipB = 2;
    /// assert_eq!(Solution::intersect(listA, listB), vec![]);
    /// // Output: No intersection
    /// ```
    /// *Explanation*: From the head of A, it reads as [2,6,4]. From the head of B, it reads as [1,5]. Since the two 
    /// lists do not intersect, intersectVal must be 0, while skipA and skipB can be arbitrary values.
    /// *Explanation*: The two lists do not intersect, so return null.
    ///
    ///
    /// Constraints:
    /// ----------------
    /// - The number of nodes of listA is in the m.
    /// - The number of nodes of listB is in the n.
    /// - 1 <= m, n <= 3 * 104
    /// - 1 <= Node.val <= 105
    /// - 0 <= skipA < m
    /// - 0 <= skipB < n
    /// - intersectVal is 0 if listA and listB do not intersect.
    /// - intersectVal == listA[skipA] == listB[skipB] if listA and listB intersect.
    /// 
    fn get_intersection_node(head_a: LinkList, head_b: LinkList) -> LinkList {
        // Get the length of both lists
        // Move the pointer of the longer list to the head of the shorter list
        // Compare 
        fn get_length(head: &LinkList) -> usize {
            match head {
                None => 0,
                Some(node) => {
                    1 + get_length(&node.borrow().next)
                }
            }
        }
        let size_a = get_length(&head_a);
        let size_b = get_length(&head_b);
        let mut curr_a = head_a.clone();
        let mut curr_b = head_b.clone();
        for i in 0..size_a.abs_diff(size_b) {
            if size_a > size_b {
                curr_a = curr_a.unwrap().borrow().next.clone();
            } else {
                curr_b = curr_b.unwrap().borrow().next.clone();
            }
        }
        while let (Some(a), Some(b)) = (curr_a.clone(), curr_b.clone()) {
            if a == b {
                return Some(a);
            }
            curr_a = a.borrow().next.clone();
            curr_b = b.borrow().next.clone();
        }
        return None;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[macro_use]
    macro_rules! list {
        () => {
            None
        };
        ($t:expr) => {
            Some(Rc::new(RefCell::new(ListNode { val: $t, next: None })))
        };
        ($t:expr, $($tail:tt)*) => {
            {
                Some(Rc::new(RefCell::new(
                    ListNode { 
                        val: $t, 
                        next: list!($( $tail )* )
                    }
                )))
            }
        }
    }

    #[test]
    fn test_1() { 
        let list_a: LinkList = list![4,1,8,4,5]; 
        let list_b: LinkList = list![5,6,1,8,4,5]; 
        let res: LinkList = Solution::get_intersection_node(list_a, list_b);
        assert_eq!(res, list![1, 8, 4,5]);
    }

    #[test]
    fn test_2() {
        let list_a: LinkList = list![2,6,4]; 
        let list_b: LinkList = list![1,5]; 
        let res: LinkList = Solution::get_intersection_node(list_a, list_b);
        assert_eq!(res, list![]);
    }

    #[test]
    fn test_3() {
        let list_a: LinkList = list![1,9,1,2,4]; 
        let list_b: LinkList = list![3,2,4]; 
        let res: LinkList = Solution::get_intersection_node(list_a, list_b);
        assert_eq!(res, list![2, 4]);
    }
}