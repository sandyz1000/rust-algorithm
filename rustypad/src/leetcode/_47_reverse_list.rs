//

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct ListNode {
    pub val: i32,
    pub next: ListLink,
}

type ListLink = Option<Box<ListNode>>;



struct Solution;

impl Solution {

    #[allow(dead_code)]
    fn reverse_list_with_memory(head: ListLink) -> ListNode {
        
        let mut curr = head;
        let mut prev = Some(Box::new(ListNode {val: -1, next: None}));

        while curr.is_some() {
            if let Some(mut node) = curr {
                // Advance the curr pointer
                curr = node.next.take();
                // Make the next pointer point to the previous node
                node.next = prev;
                prev = Some(node)
            }
        }
        
        *prev.unwrap()
    }

    /// 1 -> 2 -> 3 -> 4
    #[allow(dead_code)]
    fn reverse_list(head: ListLink) -> ListLink {
        
        let mut curr = head;
        let mut prev = None;
        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}


#[cfg(test)]
mod test {
    
    #[macro_use]
    macro_rules! list {
        () => {
            None
        };
        ($t:expr) => {
            Some(Box::new(ListNode { val: $t, next: None }))
        };
        ($t:expr, $($tail:tt)*) => {
            {
                Some(Box::new(
                    ListNode { 
                        val: $t, 
                        next: list!($( $tail )* )
                    }
                ))
            }
        }
    }

    use super::*;
    #[test]
    fn test_one() {

        let head = list!();
        let res = list!();
        assert_eq!(Solution::reverse_list(head), res);
    }

    #[test]
    fn test_two() {
        let head = list!(1);
        let res = list!(1);
        assert_eq!(Solution::reverse_list(head), res);
    }

    #[test]
    fn test_three() {
        let head = list!(1, 2);
        let res = list!(2, 1);
        assert_eq!(Solution::reverse_list(head), res);
        let head = list!(1, 2, 3);
        let res = list!(3, 2, 1);
        assert_eq!(Solution::reverse_list(head), res);
    }
}
