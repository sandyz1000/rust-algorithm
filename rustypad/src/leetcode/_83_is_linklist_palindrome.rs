#![allow(unused)]

struct Solution;
type ListLink = Option<Box<ListNode>>;

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

impl Solution {
    fn is_palindrome(&self, head: Option<Box<ListNode>>) -> bool {
        // time complexity = O(n), Space Complexity = O(1)
        // First, we get the middle element accordingly
        let (mid, even) = self.get_mid(&head);
        let mut second = mid.as_ref().unwrap().next.as_ref();
        // if, its even then ignoring the middle value
        if !even {
            second = second.unwrap().next.as_ref();
        }
        // Reversing the first half of the list
        let mut first = self.reverse(&head, &mid);
        while first.is_some() && second.is_some() {
            if first.as_ref().unwrap().val != second.as_ref().unwrap().val {
                return false;
            }
            first = first.unwrap().next;
            second = second.unwrap().next.as_ref();
        }
        true
    }
    
    fn reverse(&self, head: &Option<Box<ListNode>>, upto: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // let mut prev = ListNode::new(-1, None).as_ref();
        // let mut next = None;
        // let mut curr = head.as_ref();

        // while curr.is_some() {
        //     next = curr.as_ref().unwrap().next.as_ref();
        //     curr.as_mut().unwrap().next = prev.take();
        //     prev = curr.take();
        //     curr = next;
        // }
        // prev

        unimplemented!()
    }
    
    fn get_mid(&self, head: &Option<Box<ListNode>>) -> (Option<Box<ListNode>>, bool) {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref().unwrap().next.as_ref();
        let mut prev = head.as_ref();
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            prev = slow;
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        if fast.is_none() {
            return (prev.cloned(), false);
        }
        (slow.cloned(), true)
    }
        
}


mod tests {
    use super::*;

    fn test1() {
        // head = [1,2,2,1]
        let s = Solution {};
        let lists = linked_list!(1,2,2,1);
        assert_eq!(s.is_palindrome(lists), true);

    }

    fn test2() {
        // head = [1,2]
        let s = Solution {};
        let lists = linked_list!(1,2);
        assert_eq!(s.is_palindrome(lists), false);
    }
}