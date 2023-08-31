#![allow(unused)]

#[macro_use]
macro_rules! llist {
    () => {
        None
    };
    ($t:expr) => {
        ListLink::new($t, None)
    };
    ($t:expr, $($tail:tt)*) => {
        ListLink::new($t, llist!(
            $( $tail )* 
        ))
    }
}

use std::borrow::BorrowMut;

struct Solution;

impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode {
            next: None,
            val: val,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

type ListLink = Option<Box<ListNode>>;
trait ListInit {
    fn new(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode {
            val: val,
            next: next,
        }))
    }
}
impl ListInit for ListLink {}

impl Solution {
    /// ## 2. Add Two Numbers
    ///
    /// https://leetcode.com/problems/add-two-numbers/description/
    ///
    ///
    /// You are given two non-empty linked lists representing two non-negative integers.
    /// The digits are stored in reverse order, and each of their nodes contains a single digit. 
    /// Add the two numbers and return the sum as a linked list.
    ///
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    ///
    /// Example 1:
    /// ----------
    /// ```
    /// let l1 = vec![2,4,3]; let l2 = vec![5,6,4];
    /// let ans = vec![7,0,8];
    /// ```
    /// Explanation: 342 + 465 = 807.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let l1 = vec![0]; let l2 = vec![0];
    /// let ans  = vec![0];
    /// ```
    /// Example 3:
    /// ----------
    /// ```
    /// let l1 = vec![9,9,9,9,9,9,9]; let l2 = vec![9,9,9,9];
    /// let ans = vec![8,9,9,9,0,0,0,1];
    /// assert_eq!(Solution::add_two_numbers(l1, l2), ans);
    /// ```
    /// Constraints:
    /// ------------
    /// * The number of nodes in each linked list is in the range [1, 100].
    /// * 0 <= Node.val <= 9
    /// * It is guaranteed that the list represents a number that does not have leading zeros.
    /// 
    /// - Create a dummy list
    /// - Iterate and add two number
    fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        let mut dummy: ListNode = ListNode::new(-1);
        let mut p1: &ListLink = &l1;
        let mut p2: &ListLink = &l2;
        let mut p3 = &mut dummy;
        let mut carry: i32 = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum: i32 = carry;
            if let Some(n1) = p1 {
                sum += n1.val;
                p1 = &n1.next;
            }

            if let Some(n2) = p2 {
                sum += n2.val;
                p2 = &n2.next;
            }

            carry = sum / 10;
            p3.next = Some(Box::new(ListNode::new(sum % 10)));
            // let mut p4: &ListNode = dummy.borrow_mut();
            // let y = p4.next.as_mut().unwrap();
            p3 = p3.next.as_mut().unwrap();
        }

        dummy.next
    }

    /// Use memory to manipulate linked pointer
    #[allow(dead_code)]
    fn add_two_numbers_ii(l1: ListLink, l2: ListLink) -> ListLink {
        let mut dummy: ListLink = ListLink::new(-1, None);
        let mut p1: &ListLink = &l1;
        let mut p2: &ListLink = &l2;
        let mut p3 = dummy.borrow_mut();
        let mut carry: i32 = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut sum: i32 = carry;
            if let Some(n1) = p1 {
                sum += n1.val;
                p1 = &n1.next;
            }

            if let Some(n2) = p2 {
                sum += n2.val;
                p2 = &n2.next;
            }

            carry = sum / 10;
            if let Some(node) = p3 {
                node.next = Some(Box::new(ListNode::new(sum % 10)));
                p3 = node.next.borrow_mut();
            }
        }

        dummy.unwrap().next
    }

}



#[test]
fn test_string() {
    // Tutorial for understand and manipulate mutable reference of data !!!
    let mut var_a = String::from("Howdy!!");
    let _var_b = &var_a;
    let var_c = &mut var_a;

    var_c.push_str("Hello world!");

    print!("The string is: {}", var_c);

}

#[test]
fn test() {
    let l1 = llist!(2, 4, 3);
    let l2 = llist!(5, 6, 4);
    let l3 = llist!(7, 0, 8);

    assert_eq!(Solution::add_two_numbers_ii(l1, l2), l3);

    // let x = hmap!(
    //     "fin" => "hello",
    //     "gin" => "simsim"
    // );
}
