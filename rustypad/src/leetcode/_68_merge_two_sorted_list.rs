/* 
https://leetcode.com/problems/merge-two-sorted-lists/description/
21. Merge Two Sorted Lists

You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing 
together the nodes of the first two lists.

Example 1:
----------
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:
----------
Input: list1 = [], list2 = []
Output: []

Example 3:
----------
Input: list1 = [], list2 = [0]
Output: [0]

 */
use std::{borrow::BorrowMut};

struct Solution;

type ListOption = Option<Box<ListNode>>;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: ListOption
}

trait ListInit {
    fn new(val: i32) -> ListOption {
        Some(Box::new(ListNode {val, next: None}))
    }
}

impl ListInit for ListOption {}


impl Solution {
    
    #[allow(dead_code)]
    fn new(arr: Vec<i32>) -> ListOption {
        // Create a linklist from array
        let mut head: ListNode = ListNode { val: -1, next: None };
        let mut p: &mut ListNode = head.borrow_mut();
        for a in arr.iter() {
            p.next = ListOption::new(a.clone());
            p = p.next.as_mut().unwrap();
        }

        head.next
    }

    #[allow(dead_code)]
    fn merge_two_lists(l1: ListOption, l2: ListOption) -> ListOption {
        // Create dummy list
        // Compare if first_list value < second_list value
        // Copy value to to new list
        // TODO: Fix the test case
        let mut merge = ListOption::new(-1);
        let mut dummy: &mut Option<Box<ListNode>> = &mut merge;
        let mut first = &l1.clone();
        let mut second= &l2.clone();
        
        while first.is_some() || second.is_some() {
            if first.is_some() && second.is_none() {
                if let Some(fir) = first {
                    dummy.as_mut().unwrap().next = fir.next.clone();
                    first = &fir.next;
                }

                break;
            }
            if first.is_none() && second.is_some() {
                if let Some(sec) = second {
                    dummy.as_mut().unwrap().next = sec.next.clone();
                    second = &sec.next;
                }
                break;
            } 
            
            // If both list has value
            if first.is_some() && second.is_some() {
                if first.as_ref().unwrap().val < second.as_ref().unwrap().val {
                    if let Some(fir) = first {
                        dummy.as_mut().unwrap().next = fir.next.clone();
                        first = &fir.next;
                    }
                } 
                else {
                    if let Some(sec) = second {
                        dummy.as_mut().unwrap().next = sec.next.clone();
                        second = &sec.next;
                    }
                }
            }            
        }

        merge.unwrap().next
    }

}


#[test]
fn test() {
    let a = Solution::new(vec![1, 2, 4]);
    let b = Solution::new(vec![1, 3, 4]);
    let c = Solution::new(vec![1, 1, 2, 3, 4, 4]);
    assert_eq!(Solution::merge_two_lists(a, b), c);
}
