#![allow(unused)]

type LinkedList = Option<Box<ListNode>>;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: LinkedList
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: LinkedList) -> LinkedList {
        Some(Box::new(ListNode { val, next }))
    }
}

#[macro_use]
macro_rules! linked_list {
    () => {
        None 
    };

    ($t:expr) => {
        ListNode::new($t, None)
    };

    ($t:expr, $($tail:tt)* ) => {
        ListNode::new($t, linked_list!($($tail)*))
    };
}


struct Solution;

impl Solution {
    /// ## Swap Nodes in Pairs
    /// https://leetcode.com/problems/swap-nodes-in-pairs/
    /// 
    /// Given a linked list, swap every two adjacent nodes and return its head. You must 
    /// solve the problem without modifying the values in the list's nodes (i.e., only nodes 
    /// themselves may be changed.)
    ///
    /// Example 1:
    /// ---------
    /// 1 -- > 2 --> 3 --> 4
    /// 2 --> 1 --> 4 --> 3
    ///
    /// Input: head = [1,2,3,4]
    /// Output: [2,1,4,3]
    ///
    /// Example 2:
    /// ---------
    /// Input: head = []
    /// Output: []
    ///
    /// Example 3:
    /// ---------
    /// Input: head = [1]
    /// Output: [1]
    ///  
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [0, 100].
    /// 0 <= Node.val <= 100
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unimplemented!()    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        let node = linked_list!(1,2,3,4);
        assert_eq!(Solution::swap_pairs(node), linked_list!(2,1,4,3));
    }

    #[test]
    fn test_swap_pairs_2() {
        let node = linked_list!(1);
        assert_eq!(Solution::swap_pairs(node), linked_list!(1));
    }

    #[test]
    fn test_swap_pairs_3() {
        let node: LinkedList = None;
        assert_eq!(Solution::swap_pairs(node), None);
    }
}
