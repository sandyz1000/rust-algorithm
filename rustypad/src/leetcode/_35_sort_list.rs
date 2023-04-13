
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

use std::{borrow::BorrowMut, rc::Rc, ops::{DerefMut, Deref}};

struct Solution;

type ListOption = Option<Box<ListNode>>;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: ListOption
}

trait ListInit {
    fn new(val: i32) -> ListOption {
        Some(Box::new(ListNode {val, next: None}))
    }
}


impl Deref for ListNode {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl DerefMut for ListNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
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

    /// 1 -> 2 -> 3 -> 4
    /// Two pointer fast and mid
    /// Accept two argument mut_head for mutating to None after mid
    /// and head that will be used to find mid pointer
    #[allow(dead_code)]
    fn get_mid(head: &ListOption) -> (ListOption, ListOption) {
        let new_head: ListOption = head.clone();
        let mut mid_prev = &mut head.to_owned();
        let mut p1 = &new_head;
        
        while let Some(curr_head) = p1 {
            // Check if head is none
            if curr_head.next.is_none() {
                break
            }

            // Increament the mid_prev node
            mid_prev = &mut mid_prev.as_mut().unwrap().next;
            
            // Increament the fast pointer by 2 node
            if let Some(next_node) = &curr_head.next {
                p1 = &next_node.next;
            }
        }
        
        let mid: ListOption = mid_prev.clone().unwrap().next;
        mid_prev.as_mut().unwrap().next = None;
        
        (new_head, mid)
    }
    
    /// - create dummy node
    /// - compare and copy smaller element to dummy
    #[allow(dead_code)]
    #[allow(unused_variables)]  // This will not give us any warning for unused variable
    fn merge(head1: ListOption, head2: ListOption) -> ListOption {
        
        // Create a ref variable that hold head1 and head2
        // Compare and check if both are available and add it to dummy node
        let mut p1: &ListOption = &head1;
        let mut p2: &ListOption = &head2;

        let mut dummy: ListOption = ListOption::new(-1);
        let mut current = &mut dummy;

        while p1.is_some() && p2.is_some() {
            
            if let Some(curr) = current {
                if p1.as_ref().unwrap().val < p2.as_ref().unwrap().val {
                    if let Some(n1) = p1 {
                        curr.next = n1.next.clone();
                        p1 = &n1.next;
                    }
                }
                else {
                    if let Some(n2) = p2 {
                        curr.next = n2.next.clone();
                        p2 = &n2.next;    
                    }
                }

                current = &mut curr.next;
            }
        }

        // Copy rest of the element to dummy
        while let Some(n1) = p1 {
            if let Some(curr) = current {
                curr.next = n1.next.clone();
                p1 = &n1.next;
                current = &mut curr.next;
            }
        }

        while let Some(n2) = p2 {
            if let Some(curr) = current {
                curr.next = n2.next.clone();
                p2 = &n2.next;
                current = &mut curr.next;
            }
        }

        dummy?.next

    }

    /// Merge sort
    /// - Find mid
    /// - recurse to sortList for left and right 
    /// - merge sort list
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn sort_list(head: &ListOption) -> ListOption {
        // If head is none
        if head.is_none() {
            return None;
        }

        // If head.next is none then return head
        if let Some(node) = head {
            if node.next.is_none() {
                return Some(node.clone());
            }
        }
        
        // Create a mutable burrow from immutable head
        
        let (new_head, mid) = Solution::get_mid(head);
        
        let left = Solution::sort_list(&new_head);
        let right = Solution::sort_list(&mid);
        return Solution::merge(left, right);
    }

}

// TODO: Fix Tests
#[test]
#[allow(unused_variables)]
fn test1() {
    let arr = vec![4, 2, 1, 3];
    let head1: ListOption = Solution::new(arr);
    let output = Solution::sort_list(&head1);
    // assert!(output, init(sorted(arr)))
}

#[test]
#[allow(unused_variables)]
fn test2() {
    let arr = vec![-1, 5, 3, 4, 0];
    let head2 = Solution::new(arr);
    let output = Solution::sort_list(&head2);
    // assert output == init([-1, 0, 3, 4, 5])
}

#[test]
#[allow(unused_variables)]
fn test_string() {
    let sentence = "The fox jumps over the dog";
    let index = sentence.find("fox");

    // let words_after_fox = &sentence[index..]; // Error: Can't index str with Option<usize>

    if let Some(fox) = index {
        let words_after_fox = &sentence[fox..];
        println!("{}", words_after_fox);
    }
}