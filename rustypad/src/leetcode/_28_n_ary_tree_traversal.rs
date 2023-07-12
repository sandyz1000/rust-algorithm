#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;


// type NodeRef = Rc<RefCell<Node>>;
type NodeRef = Box<Node>;

struct Node {    
    val: i32,
    children: Vec<NodeRef>,
}

impl Node {
    fn new(val: i32) -> NodeRef {
        Box::new(Self {
            val, 
            children: Vec::new()
        })
    }

    fn new_with_children(val: i32, children: Vec<NodeRef>) -> NodeRef {
        Box::new(Self {
            val,
            children
        })
    }
}

struct Solution;

impl Solution {
    /// ## 429. N-ary Tree Level Order Traversal
    /// https://leetcode.com/problems/n-ary-tree-level-order-traversal/description/
    ///
    /// Given an n-ary tree, return the level order traversal of its nodes' values.
    ///
    /// Nary-Tree input serialization is represented in their level order traversal, each group of children 
    /// is separated by the null value (See examples).
    ///
    /// Example 1:
    /// ---------
    /// ```doc
    /// Input: root = [1,null,3,2,4,null,5,6]
    /// Output: [[1],[3,2,4],[5,6]]
    /// ```
    /// Example 2:
    /// ----------
    /// ```doc
    /// Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]
    /// Output: [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]```
    /// ```
    ///
    /// Constraints:
    /// -----------
    /// * The height of the n-ary tree is less than or equal to 1000
    /// * The total number of nodes is between [0, 104]
    pub fn level_order(root: Option<NodeRef>) -> Vec<Vec<i32>> {
        unimplemented!()
    }

}

macro_rules! ntree {
    ($t:expr) => {
        Node::new($t)
    };
    ($t:expr, $( $tr:tt )*, ) => {
        {
            let mut node = Node::new($t);
            $(
                node.children.push(ntree!($tr));
            )*
        }
    };
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let mut root: NodeRef = Node::new(1);
        root.children.extend([
            Node::new(3), Node::new(2), Node::new(4)
        ]);
        root.children[0].children.extend([Node::new(5), Node::new(6)]);

        let res = Solution::level_order(Some(root));
        let ans = vec![vec![1],vec![3,2,4],vec![5,6]];
        assert_eq!(res, ans);
    }

    #[test]
    fn test2() {
        let mut root: NodeRef = Node::new(1);
        root.children.extend([
            Node::new(2), Node::new(3), Node::new(4), Node::new(5)
        ]);

        root.children[1].children.extend([
            Node::new(6), 
            Node::new_with_children(
                7, 
                vec![Node::new_with_children(11, vec![Node::new(14)])]
            )
        ]);

        root.children[2].children.extend([
            Node::new_with_children(
                4, 
                vec![Node::new_with_children(8, vec![Node::new(12)])])
        ]);

        root.children[3].children.extend([
            Node::new_with_children(
                9, 
                vec![Node::new(13)]), Node::new(10)
        ]);

        let res = Solution::level_order(Some(root));
        let ans = vec![vec![1],vec![2,3,4,5],vec![6,7,8,9,10],vec![11,12,13],vec![14]];
        assert_eq!(res, ans);
    }
}