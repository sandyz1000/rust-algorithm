
#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Definition for singly-linked list.
struct Solution;

type ListLink = Option<Box<ListNode>>;

trait ListInit {
    fn new(val: i32) -> ListLink {
        Some(Box::new(ListNode { val, next: None }))
    }
}

impl ListInit for ListLink {}

#[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
struct ListNode {
    val: i32,
    next: ListLink,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    fn from_vec_vec(vec_lists: Vec<Vec<i32>>) -> Vec<ListLink> {
		let mut lists: Vec<ListLink> = vec![];
		for list in vec_lists {
			lists.push(Solution::new(list));
		}
        
		lists
    }

    fn new(lists: Vec<i32>) -> ListLink {
		let dummy = ListLink::new(-1);
		let mut curr = dummy.clone();
        
		for i in 0..lists.len()  {
			if let Some(mut node) = curr {
				node.next = ListLink::new(lists[i]);
				curr = node.next.clone();
			}
		}

		dummy.unwrap().next
    }

	/// ## Merge k Sorted Lists
	///
	/// https://leetcode.com/problems/merge-k-sorted-lists/description/
	///
	/// You are given an array of k linked-lists lists, each linked-list is sorted in
	/// ascending order.
	///
	/// Merge all the linked-lists into one sorted linked-list and return it.
	///
	/// Example 1:
	/// ----------
	/// Input: lists = [[1,4,5],[1,3,4],[2,6]]
	/// Output: [1,1,2,3,4,4,5,6]
	/// Explanation: The linked-lists are:
	/// [
	///   1->4->5,
	///   1->3->4,
	///   2->6
	/// ]
	/// merging them into one sorted list:
	/// 1->1->2->3->4->4->5->6
	///
	/// Example 2:
	/// ----------
	/// Input: lists = []
	/// Output: []
	///
	/// Example 3:
	/// ----------
	/// Input: lists = [[]]
	/// Output: []
	/// 
	/// - Use heap to add k node in the heap
	/// - Get the min and add the value to dummy and move to next pointer iff next is present
	/// - Time Complexity: O( n*k log K)
    fn merge_k_lists(lists: Vec<ListLink>) -> ListLink {
		// Add a node copy to the heap
		let mut _heapq = BinaryHeap::<(Reverse<i32>, ListLink)>::new();
		// Iterate and add first item from vec to min-heap
		for k in 0..lists.len() {
			if let Some(x) = lists[0].clone() {
				_heapq.push((Reverse(x.val), x.next));
			}
		}
		
		// Increment the pointer and push next item to heap
		let mut head: ListLink = ListLink::new(-1);
		let mut p1 = &mut head;

		while !_heapq.is_empty() {
			let (_, min_item) = _heapq.pop().unwrap();
			if let Some(node2) = min_item {
				if let Some(node1) = p1 {
					node1.next = Some(node2.clone());
					p1 = &mut node1.next;
				}
				_heapq.push((Reverse(node2.val), node2.next));
			}
		}

		head.unwrap().next
    }
}

// TODO: Fix tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let vec_vec = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let lists = Solution::from_vec_vec(vec_vec);
        let res = Solution::merge_k_lists(lists);

        assert_eq!(res, Solution::new(vec![1, 1, 2, 3, 4, 4, 5, 6]));
    }

    #[test]
    fn test2() {
        let lists = Solution::from_vec_vec(vec![]);

        let res = Solution::merge_k_lists(vec![]);
        assert_eq!(res, Solution::new(vec![]));
    }
}
