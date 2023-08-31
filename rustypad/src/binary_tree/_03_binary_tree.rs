// TODO: Build binary tree with generic treenode and refcell
#![allow(dead_code)]
// Definition for a binary tree node.


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

type NodeRef = Rc<RefCell<TreeNode>>;
type TreeLink = Option<NodeRef>;


trait TreeMaker {
    fn new(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(
            TreeNode{ val, left, right }
        )))
    }

    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(
            TreeNode { val, left: None, right: None 
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
struct Solution;

impl Solution {

    pub fn inorder_successor<T: TreeMaker>(root: T, p: T) -> T {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
}