/* 
https://leetcode.com/problems/inorder-successor-in-bst/description/

285. Inorder Successor in BST

Given the root of a binary search tree and a node p in it, return the in-order successor 
of that node in the BST. If the given node has no in-order successor in the tree, return null.

The successor of a node p is the node with the smallest key greater than p.val.

Example 1:
---------
Input: root = [2,1,3], p = 1
Output: 2
Explanation: 1's in-order successor node is 2. Note that both p and the return value is of TreeNode type.
        2
      /  \
    1     3

Example 2:
---------
Input: root = [5,3,6,2,4,null,null,1], p = 6
Output: null
Explanation: There is no in-order successor of the current node, so the answer is null.

        5
       / \
      3   6  
     / \
    2   4
   /
  1

 */
#![allow(dead_code)]


use std::rc::Rc;
use std::cell::RefCell;


type NodeRef = Rc<RefCell<BinaryTreeNode>>;
type TreeLink = Option<NodeRef>;

struct Solution;

// Definition for a binary tree node.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BinaryTreeNode {
  val: i32,
  left: TreeLink,
  right: TreeLink,
}


impl BinaryTreeNode {
  fn new(val: i32) -> Self {
    BinaryTreeNode {
      val,
      left: None,
      right: None
    }
  }
}

trait TreeMaker {
    fn new(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(
            BinaryTreeNode{ val, left, right }
        )))
    }

    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(
            BinaryTreeNode { val, left: None, right: None 
        })))
    }
}

impl TreeMaker for TreeLink {}


macro_rules! tree {
    ($e:expr, $left:expr, $right:expr) => {
        TreeLink::new($e, $left, $right)
    };

    ($e:expr) => {
        TreeLink::leaf($e)
    };
}

impl Solution {

    pub fn inorder(root: &TreeLink, p: i32, successor: &mut TreeLink) {
        
        if let Some(node_cell) = root {
            let node = node_cell.borrow();
            
            // Traverse the left subtree
            Solution::inorder(&node.left, p, successor);
            
            if successor.is_none() && node.val > p {
                *successor = tree!(node.val);
            }
            
            // Traverse the right subtree
            Solution::inorder(&node.right, p, successor);
        }

    }

    pub fn inorder_successor(root: TreeLink, p: TreeLink) -> TreeLink {
        let node_cell = p.as_ref().unwrap();
        let p = node_cell.borrow().val;

        let mut res = None;
        Solution::inorder(&root, p, &mut res);
        res
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn test0() {
        let root = tree!(2, tree!(1), tree!(3));
        let p = tree!(1);
        let res = tree!(2);
        assert_eq!(Solution::inorder_successor(root, p), res);
        let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
        let p = tree!(6);
        let res = None;
        assert_eq!(Solution::inorder_successor(root, p), res);
    }


    #[test]
    fn test1() {
        let root = vec![2,1,3];
        let p = 1;

        let left = TreeLink::leaf(1);
        let right = TreeLink::leaf(3);
        let root = TreeLink::new(2, left, right);
    
        let p = TreeLink::leaf(1);
        let res = TreeLink::leaf(2);
        assert_eq!(Solution::inorder_successor(root, p), res);

    }

    #[test]
    fn test2() {
        let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
        let p = tree!(6);
        let res = None;
        assert_eq!(Solution::inorder_successor(root, p), res);
    }

    #[test]
    fn test_refcell() {
        let cv = &Rc::new(RefCell::new(
            BinaryTreeNode {val: 5, left: None, right: None} 
        ));
        let val = cv.borrow().val;
        assert_eq!(val, 5);
    }

}