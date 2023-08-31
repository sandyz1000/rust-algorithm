#![allow(unused)]

use std::{rc::Rc, collections::BinaryHeap, cmp::Reverse, clone};
// use std::collections::LinkedList;

type ListLink = Option<Box<ListNode>>;

#[macro_use]
macro_rules! linked_list {
    () => {
        None
    };
    ($t:expr) => {
        ListNode::new($t, None)
    };
    ($t:expr, $($tail:tt)*) => {
        ListNode::new($t, linked_list!(
            $( $tail )* 
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct ListNode {
    val: i32,
    next: ListLink,
}


impl ListNode {
    fn new(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode { val, next }))
    }
}

// Let prev_node be the node at position i - 1
fn delete_node(prev_node: &mut ListNode) {
    let next_node: ListLink = match prev_node.next.as_mut() {
        Some(n1) => n1.next.take(),
        None => None,
    };
    prev_node.next = next_node;
}

#[test]
fn main() {
    let head = linked_list!(1, 2, 3, 4, 5);
    println!("{:?}", head);
}

struct Solution;

impl Solution {
    /// Given the head of a singly linked list, reverse the list, and return the reversed list. 
    pub fn reverse_list(head: ListLink) -> ListLink {
        let mut prev: ListLink = None;
        let mut curr: ListLink = head;
        while let Some(mut curr_node) = curr {
            let next_node: ListLink = curr_node.next.take();
            curr_node.next = prev;
            prev = Some(curr_node);
            curr = next_node;
        }
        prev
    }
    
    
    /// ## Given the head of a singly linked list, return the middle node of the linked list.
    ///
    /// If there are two middle nodes, return the second middle node.
    ///
    /// Example 1:
    /// ---------
    /// Input: head = [1,2,3,4,5]
    /// Output: [3,4,5]
    /// Explanation: The middle node of the list is node 3.
    ///
    /// Example 2:
    /// ---------
    /// Input: head = [1,2,3,4,5,6]
    /// Output: [4,5,6]
    /// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [1, 100].
    /// 1 <= Node.val <= 100
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn internal_ref_opt(node: ListLink) -> ListLink {
            match node {
                Some(n) => Some(Box::new(*n)),
                None => None,
            }
        }
        fn middle<'a>(head: &'a Option<Box<ListNode>>) -> &'a ListLink {
            let mut slow = head;
            let mut fast = head;

            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                slow = &slow.as_ref().unwrap().next;
                fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            }
            slow
        }
        
        let middle: &ListLink = middle(&head);
        middle.to_owned()
    }

    /// Given the head of a linked list and an integer k, return the kth node from the end.
    /// For example, given the linked list that represents 1 -> 2 -> 3 -> 4 -> 5 and k = 2, 
    /// return the node with value 4, as it is the 2nd node from the end.
    pub fn return_kth_node(head: ListLink, k: i32) -> ListLink { 
        // Use fast pointer to find the kth node
        // Use slow pointer from the start till (n - k)th node is found
        unimplemented!() 
    }

    /// ## 2130. Maximum Twin Sum of a Linked List
    /// https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/
    ///
    /// In a linked list of size n, where n is even, the ith node (0-indexed) of the linked list 
    /// is known as the twin of the (n-1-i)th node, if 0 <= i <= (n / 2) - 1.
    ///
    /// For example, if n = 4, then node 0 is the twin of node 3, and node 1 is the twin of node 2. 
    /// These are the only nodes with twins for n = 4. The twin sum is defined as the sum of a node and its twin.
    ///
    /// Given the head of a linked list with even length, return the maximum twin sum of the linked list.
    ///
    /// Example 1:
    /// ----------
    /// Input: head = [5,4,2,1]
    /// Output: 6
    /// Explanation:
    /// Nodes 0 and 1 are the twins of nodes 3 and 2, respectively. All have twin sum = 6.
    /// There are no other nodes with twins in the linked list.
    /// Thus, the maximum twin sum of the linked list is 6. 
    ///
    /// Example 2:
    /// ----------
    /// Input: head = [4,2,2,3]
    /// Output: 7
    /// Explanation:
    /// The nodes with twins present in this linked list are:
    /// - Node 0 is the twin of node 3 having a twin sum of 4 + 3 = 7.
    /// - Node 1 is the twin of node 2 having a twin sum of 2 + 2 = 4.
    /// Thus, the maximum twin sum of the linked list is max(7, 4) = 7. 
    ///
    /// Example 3:
    /// ----------
    /// Input: head = [1,100000]
    /// Output: 100001
    /// Explanation:
    /// There is only one node with a twin in the linked list having twin sum of 1 + 100000 = 100001.
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is an even integer in the range [2, 105].
    /// 1 <= Node.val <= 105
    /// 
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        unimplemented!()    
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
    /// 
    pub fn merge_k_lists(lists: Vec<ListLink>) -> ListLink {
        // Add the 1st node from the list to the heap
        // Get the min and add the value to dummy and move to next pointer iff next is present

        let mut heap: BinaryHeap<(Reverse<i32>, ListLink)> = BinaryHeap::new();
        for k in 0..lists.len() {
            if let Some(x) = lists[0].clone() {
                heap.push((Reverse(x.val), x.next));
			}
		}
        
        // Add the remaining nodes to the heap
        let head = ListNode::new(-1, None);
        let mut dummy = head.clone();
        
        while let Some((Reverse(val), next)) = heap.pop() {
            // increment the dummy node
            if let Some(mut node) = dummy {
                node.next = ListNode::new(val, None);
                dummy = node.next;
                // print!("{:?}", dummy);
            }
            // Add the next node to the heap
            if let Some(next_node) = next {
                heap.push((Reverse(next_node.val), next_node.next));
            }
        }
        head.unwrap().next
    }

    /// ## 234. Palindrome Linked List
    /// https://leetcode.com/problems/palindrome-linked-list/description/
    ///
    /// Given the head of a singly linked list, return true if it is a palindrome or false otherwise.
    ///
    /// Example 1:
    /// ----------
    /// Input: head = [1,2,2,1]
    /// Output: true
    ///
    /// Example 2:
    /// ----------
    /// Input: head = [1,2]
    /// Output: false
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [1, 105].
    /// 0 <= Node.val <= 9
    //
    // Follow up: Could you do it in O(n) time and O(1) space?
    pub fn is_palindrome(head: ListLink) -> bool {
        let mut ans: Vec<i32> = vec![];
        let mut dummy = &head;
        while let Some(node) = dummy.as_ref() {
            ans.push(node.val);
            dummy = &node.next;
        }

        let mut size = 0;
        while size < ans.len() / 2  {
            if ans[size] != ans[ans.len() - size - 1] {
                return false;
            }
            size += 1;
        }
        true
    }

    pub fn is_palindrome_optimized(head: ListLink) -> bool {
        fn middle<'a>(head: &'a Option<Box<ListNode>>) -> &'a ListLink {
            let mut slow = head;
            let mut fast = head;

            while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
                if let Some(f) = &fast.as_ref().unwrap().next {
                    fast = &f.next;
                    if let Some(s) = slow {
                        slow = &s.next;
                    }
                }
            }
            slow
        }

        // fn reverse(head: ListLink) -> ListLink {
        //     let mut prev: ListLink = ListNode::new(-1, None);
        //     let mut curr: ListLink = head;
        //     while let Some(mut curr_node) = curr {
        //         let next_node: ListLink = curr_node.next.take();
        //         curr_node.next = prev;
        //         prev = Some(curr_node);
        //         curr = next_node;
        //     }
        //     prev
        // }


        // Reverse the second half of the list and compare with the first half
        let mut dummy: &ListLink = &head;
        let mut mid = middle(&head);
        let mut mid = Solution::reverse_list(mid.clone());

        while let Some(node) = mid {
            if node.val != dummy.as_ref().unwrap().val {
                return false;
            }
            if let Some(d) = dummy {
                dummy = &d.next;
            }
            mid = node.next;
        }
        true
    }

    /// ## 83. Remove Duplicates from Sorted List
    ///
    /// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. 
    /// Return the linked list sorted as well.
    ///
    /// Example 1:
    /// ----------
    /// - Input: head = [1,1,2]
    /// - Output: [1,2]
    ///
    /// Example 2:
    /// ----------
    /// - Input: head = [1,1,2,3,3] 
    /// - Output: [1,2,3]
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [0, 300].
    /// -100 <= Node.val <= 100
    /// The list is guaranteed to be sorted in ascending order.
    ///
    /// NOTE: Use mut head when you need to edit the head of the linked list
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Traverse and check if node.next  == node
        let mut curr_opt = head.as_mut();
        
        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();
            
            // If next is present and next.val == curr.val, increment next pointer
            while let Some(next) = next_opt.as_mut() {    
                if next.val == curr.val { 
                    next_opt  = next.next.take(); 
                }
                else { 
                    curr.next = next_opt;  
                    break; 
                }
            }
            curr_opt = curr.next.as_mut();
        }
        head
    }

    /// ## 24. Swap Nodes in Pairs
    /// https://leetcode.com/problems/swap-nodes-in-pairs/
    /// 
    /// Given a linked list, swap every two adjacent nodes and return its head. 
    /// You must solve the problem without modifying the values in the list's nodes 
    /// (i.e., only nodes themselves may be changed.)
    ///
    /// Example 1:
    /// ----------
    /// - Input: head = [1,2,3,4]
    /// - Output: [2,1,4,3]
    /// 
    /// Example 2:
    /// ----------
    /// - Input: head = []
    /// - Output: []
    /// 
    /// Example 3:
    /// ----------
    /// Input: head = [1]
    /// Output: [1]
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is in the range [0, 100].
    /// 0 <= Node.val <= 100
    ///
    pub fn swap_pairs(head: ListLink) -> ListLink {
        // Get the first pair of nodes that will be swapped
        // save the second next to temporary node
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut pre = &mut dummy;

        while let Some(mut fir) = pre.next.take() {
            if let Some(mut sec) = fir.next.take() {
                // If no of node is even
                fir.next = sec.next.take();
                sec.next = Some(fir);
                pre.next = Some(sec);

                pre = pre.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                // If no of node is odd
                pre.next = Some(fir);
                pre = pre.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
    
    /// ## 25. Reverse Linked List 
    fn reverse(head: ListLink) -> ListLink {
        let dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut pre = None;
        let mut curr = head;

        while let Some(mut c1) = curr {
            // Save the next pointer
            let next = c1.next.take();
            // Make the next to curr to previous
            c1.next = pre.take();
            // Make curr to next
            curr = next;
            // Set the previous to the current
            pre = Some(c1);
        }

        pre
    }

    /// ## 92. Reverse Linked List II
    ///
    /// Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes 
    /// of the list from position left to position right, and return the reversed list.
    ///
    /// Example 1:
    /// ----------
    /// Input: head = [1,2,3,4,5], left = 2, right = 4
    /// Output: [1,4,3,2,5]
    ///
    /// Example 2:
    /// ----------
    /// Input: head = [5], left = 1, right = 1
    /// Output: [5]
    ///
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the list is n.
    /// 1 <= n <= 500
    /// -500 <= Node.val <= 500
    /// 1 <= left <= right <= n
    ///
    /// Follow up: Could you do it in one pass?
    fn reverse_between_onepass(head: ListLink, left: i32, right: i32) -> ListLink {
        // Find the left and right nodes in the list
        let mut dummy = Some(Box::new(ListNode {
            val: 0,
            next: head,
        }));
        let mut before = &mut dummy;
        for _ in 1..left {
            before = &mut before.as_mut()?.next;
        }
        
        let mut node = before.as_mut()?.next.take();
        let mut node2 = node.as_mut()?.next.take();
        for _ in left..right {
            let node3 = node2.as_mut()?.next.take();
            node2.as_mut()?.next = node;
            node = node2;
            node2 = node3;
        }
        
        let mut rev_tail = &mut node;
        for _ in left..right {
            rev_tail = &mut rev_tail.as_mut()?.next;
        }
        rev_tail.as_mut()?.next = node2;
        before.as_mut()?.next = node;
        
        dummy.unwrap().next
    }
   
    /// ## 92. Reverse Linked List II
    fn reverse_between(head: ListLink, left: i32, right: i32) -> ListLink {
        // Use vector to save the nodes in the list
        // Reverse from left to right
        // Copy to ans ListNode
        if left == right { return head }
        let mut ans: Vec<i32> = vec![];
        let mut next = head;
        while let Some(node) = next {
            ans.push(node.val);
            next = node.next;
        }

        // Reverse the vector inplace in that position
        ans[(left as usize - 1)..=(right as usize - 1)].reverse();

        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });
        let mut p1 = &mut dummy; 

        for i in ans {
            p1.next = Some(Box::new(ListNode {
                val: i,
                next: None,
            }));
            p1 = p1.next.as_mut()?;
        }
        dummy.next
    }

    /// ## 2. Add Two Numbers
    ///
    /// You are given two non-empty linked lists representing two non-negative integers. The digits are stored 
    /// in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the 
    /// sum as a linked list.
    ///
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: l1 = [2,4,3], l2 = [5,6,4]
    /// - Output: [7,0,8]
    /// Explanation: 342 + 465 = 807.
    ///
    /// Example 2:
    /// ----------
    /// - Input: l1 = [0], l2 = [0]
    /// - Output: [0]
    ///
    /// Example 3:
    /// ----------
    /// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    /// Output: [8,9,9,9,0,0,0,1]
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in each linked list is in the range [1, 100].
    /// 0 <= Node.val <= 9
    /// It is guaranteed that the list represents a number that does not have leading zeros.
    ///
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        // Create result ListNode p3
        // When both the node value is present, add the value to p3
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None
        });
        let mut p3 = &mut dummy;
        let mut carry: i32 = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = p1 {
                sum += node.val;
                p1 = node.next.as_ref();
            }

            if let Some(node) = p2 {
                sum += node.val;
                p2 = node.next.as_ref();    
            }
            
            carry = sum / 10;
            
            p3.next = Some(Box::new(ListNode {
                val: sum % 10,
                next: None
            }));
            p3 = p3.next.as_mut().unwrap();
        }   

        dummy.next
    }

    /// ## 21. Merge Two Sorted Lists
    /// You are given the heads of two sorted linked lists list1 and list2.
    ///
    /// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes 
    /// of the first two lists.
    ///
    /// Return the head of the merged linked list.
    ///
    /// Example 1:
    /// ----------
    /// - Input: list1 = [1,2,4], list2 = [1,3,4]
    /// - Output: [1,1,2,3,4,4]
    ///
    /// Example 2:
    /// ----------
    /// - Input: list1 = [], list2 = []
    /// - Output: []
    ///
    /// Example 3:
    /// ----------
    /// - Input: list1 = [], list2 = [0]
    /// - Output: [0]
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in both lists is in the range [0, 50].
    /// -100 <= Node.val <= 100
    /// Both list1 and list2 are sorted in non-decreasing order.
    ///
    pub fn merge_two_lists(list1: ListLink, list2: ListLink) -> ListLink {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None
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
                        next: None
                    }));
                    p1 = p1?.next.as_ref();
                } else {
                    p3.next = Some(Box::new(ListNode {
                        val: p2.unwrap().val,
                        next: None
                    }));
                    p2 = p2?.next.as_ref();
                }
            }
            p3 = p3.next.as_mut().unwrap();
        }

        // When one of the node value is present, add the value to p3

        if p1.is_some() && p2.is_none() {
            p3.next = Some(p1?.clone());
        }
        else if p2.is_some() && p1.is_none() {
            p3.next = Some(p2?.clone());
        }

        dummy.next
    }

    /// ## 328. Odd Even Linked List
    /// https://leetcode.com/problems/odd-even-linked-list/description/
    /// 
    /// Given the head of a singly linked list, group all the nodes with odd indices together followed by the 
    /// nodes with even indices, and return the reordered list.
    ///
    /// The first node is considered odd, and the second node is even, and so on.
    ///
    /// Note that the relative order inside both the even and odd groups should remain as it was in the input.
    ///
    /// You must solve the problem in O(1) extra space complexity and O(n) time complexity.
    ///
    /// Example 1:
    /// ----------
    /// - Input: head = [1,2,3,4,5]
    /// - Output: [1,3,5,2,4]
    ///
    /// Example 2:
    /// ---------
    /// - Input: head = [2,1,3,5,6,4,7]
    /// - Output: [2,3,6,7,1,5,4]
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in the linked list is in the range [0, 104].
    /// -106 <= Node.val <= 106
    ///
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        unimplemented!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merge_two_lists() {
        let l1: ListLink = linked_list!(1,2,4);
        let l2: ListLink = linked_list!(1,3,4);
        let l3: ListLink = linked_list!(1,1,2,3,4,4);
        assert_eq!(Solution::merge_two_lists(l1, l2), l3);

        let l1: ListLink = None;
        let l2: ListLink = linked_list!(0);
        let l3: ListLink = linked_list!(0);
        assert_eq!(Solution::merge_two_lists(l1, l2), l3);
    }

    #[test]
    fn test_pair_sum() {
        let head = linked_list!(5,4,2,1);
        assert_eq!(Solution::pair_sum(head), 6);

        let head = linked_list!(4,2,2,3);
        assert_eq!(Solution::pair_sum(head), 7);

        let head = linked_list!(1,100000);
        assert_eq!(Solution::pair_sum(head), 100001);
    }

    #[test]
    fn test_merge_k_lists() {
        let lists = vec![
            linked_list!(1,4,5), 
            linked_list!(1,3,4), 
            linked_list!(2,6)
        ];
        let res = Solution::merge_k_lists(lists);
        assert_eq!(res, linked_list!(1,1,2,3,4,4,5,6));

        let lists = vec![];
        assert_eq!(Solution::merge_k_lists(lists), linked_list!(1));

    }

    #[test]
    fn test_is_palindrome() {
        // head = [1,2,2,1]
        let lists = linked_list!(1,2,2,1);
        assert_eq!(Solution::is_palindrome_optimized(lists), true);

        // head = [1,2]
        let lists = linked_list!(1,2);
        assert_eq!(Solution::is_palindrome_optimized(lists), false);
    }

}

// fn main() {
//     let a: Option<i32> = Some(5);
//     let b: Option<i32> = Some(5);

//     if a == b {
//         println!("a and b are equal");
//     } else {
//         println!("a and b are not equal");
//     }
// }