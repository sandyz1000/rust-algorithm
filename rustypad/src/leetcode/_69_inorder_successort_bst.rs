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

struct Solution;

type TreeLink = Option<Box<TreeNode>>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink
}

trait TreeMaker {
    fn new(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Box::new(
            TreeNode{ val, left, right }
        ))
    }

    fn leaf(val: i32) -> TreeLink {
        Some(Box::new(TreeNode { val, left: None, right: None }))
    }
}

impl TreeMaker for TreeLink {}


impl Solution {

    #[allow(dead_code)]
    fn inorder_successor(root: &TreeLink, p: &TreeLink) -> TreeLink {
        // recurse to left child
        // if successor found return 
        // if root.val > p; then return root
        // recurse to right child
        if root.is_none() {
            return None;
        }

        let left = &root.as_ref().unwrap().left;
        let successor = Solution::inorder_successor(left, p);
        if successor.is_some() {
            return successor;
        }

        if let (Some(n1), Some(n2)) = (root, p) {
            if n1.val > n2.val {
                return TreeLink::leaf(n1.val);
            }
        }
        
        let right = &root.as_ref().unwrap().right;
        Solution::inorder_successor(right, p)
    }

}

#[macro_export]
macro_rules! tree {
    ($e:expr, $left:expr, $right:expr) => {
        TreeLink::new($e, $left, $right)
    };

    ($e:expr) => {
        TreeLink::leaf($e)
    };
}

// TODO: Fix Test
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test1() {
        let left = TreeLink::leaf(1);
        let right = TreeLink::leaf(3);
        let root = TreeLink::new(2, left, right);
    
        let p = TreeLink::leaf(1);
        let res = TreeLink::leaf(2);
        assert_eq!(Solution::inorder_successor(&root, &p), res);
        
    }
    
    #[test]
    fn test2() {
        let root = tree!(5, tree!(3, tree!(2, tree!(1), None), tree!(4)), tree!(6));
        let p = tree!(6);
        let res = None;
        assert_eq!(Solution::inorder_successor(&root, &p), res);
    }
    
}
