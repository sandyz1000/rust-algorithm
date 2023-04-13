//

use crate::llist;


impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode { next: None, val: val }
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
        Some(Box::new(ListNode { val: val, next: next }))
    }
}
impl ListInit for ListLink { }


struct Solution;

impl Solution {

    #[allow(dead_code)]
    fn reverse_list_with_memory(head: ListLink) -> ListNode {
        
        let mut next_node = head;
        let mut prev = Some(Box::new(ListNode::new(-1)));

        while next_node.is_some() {
            if let Some(mut node) = next_node {
                next_node = node.next;
                node.next = prev;
                prev = Some(node)
            }
        }
        
        *prev.unwrap()
    }

    /// 1 -> 2 -> 3 -> 4
    #[allow(dead_code)]
    fn reverse_list(head: ListLink) -> ListLink {
        
        let mut next_node = head;
        let mut prev = None;
        while let Some(mut node) = next_node {
            next_node = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}



#[test]
fn test() {
    let head = llist!();
    let res = llist!();
    assert_eq!(Solution::reverse_list(head), res);
    let head = llist!(1);
    let res = llist!(1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = llist!(1, 2);
    let res = llist!(2, 1);
    assert_eq!(Solution::reverse_list(head), res);
    let head = llist!(1, 2, 3);
    let res = llist!(3, 2, 1);
    assert_eq!(Solution::reverse_list(head), res);
}
