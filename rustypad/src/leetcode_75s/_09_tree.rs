#![allow(unused)]
use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;

struct Solution;
type NodeRef = Rc<RefCell<TreeNode>>;
type NodeOption = Option<NodeRef>;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
  pub val: i32,
  pub left: Option<NodeRef>,
  pub right: Option<NodeRef>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<NodeRef>, right: Option<NodeRef>) -> Option<NodeRef> {
        Some(Rc::new(RefCell::new(
            TreeNode {
                val,
                left,
                right
            }
        )))
    }
}

#[macro_use]
macro_rules! tree_node {
    ($t:expr) => {
        TreeNode::new($t, None, None)
    };

    ($t:expr, $left:expr, $right:expr) => {
        TreeNode::new($t, $left, $right)
    }
}

impl Solution {
    /// ## 104. Maximum Depth of Binary Tree
    /// Given the root of a binary tree, return its maximum depth.
    ///
    /// A binary tree's maximum depth is the number of nodes along the longest path from the root 
    /// node down to the farthest leaf node.
    ///
    /// Example 1:
    /// ----------
    /// Input: root = [3,9,20,null,null,15,7]
    /// Output: 3
    ///
    /// Example 2:
    /// ----------
    /// Input: root = [1,null,2]
    /// Output: 2
    ///
    /// Constraints:
    /// ----------
    /// The number of nodes in the tree is in the range [0, 104].
    /// -100 <= Node.val <= 100
    fn max_depth(root: Option<NodeRef>) -> i32 {
        fn _recurse_max_depth(root: &Option<NodeRef>) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let left_max: i32 = _recurse_max_depth(&node.borrow().left);
                    let right_max: i32 = _recurse_max_depth(&node.borrow().right);
                    1 + std::cmp::max(left_max, right_max)
                }
            }
        }

        fn _iterative_max_depth(root: &Option<NodeRef>) -> i32 {
            // Save the current node and depth in a stack.
            if root.is_none() {
                return 0;
            }
            let mut ans: i32 = 0;
            let mut stack: Vec<(NodeOption, i32)> = vec![(root.clone(), 1)];
            while !stack.is_empty() {
                let (node, depth) = stack.pop().unwrap();
                ans = std::cmp::max(ans, depth);
                
                if node.is_some() {
                    if let Some(n) = node {
                        if n.borrow().left.is_some() {
                            stack.push((n.borrow().left.clone(), depth + 1));
                        }
                        if n.borrow().right.is_some() {
                            stack.push((n.borrow().right.clone(), depth + 1));
                        }   
                    }
                }
            }
            ans
        }

        _recurse_max_depth(&root)
    }

    /// ## 226. Invert Binary Tree
    /// https://leetcode.com/problems/invert-binary-tree/
    ///
    /// Given the root of a binary tree, invert the tree, and return its root.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: root = [4,2,7,1,3,6,9]
    /// - Output: [4,7,2,9,6,3,1]
    ///
    /// Example 2:
    /// ----------
    /// - Input: root = [2,1,3]
    /// - Output: [2,3,1]
    ///
    /// Example 3:
    /// ---------
    /// - Input: root = []
    /// - Output: []
    ///
    ///
    /// Constraints:
    /// ----------
    /// The number of nodes in the tree is in the range [0, 100].
    /// -100 <= Node.val <= 100
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        unimplemented!()   
    }

    /// ## 100. Same Tree
    ///
    /// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
    ///
    /// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
    ///
    /// Example 1:
    /// ----------
    /// - Input: p = [1,2,3], q = [1,2,3]
    /// - Output: true
    ///
    /// Example 2:
    /// ----------
    /// - Input: p = [1,2], q = [1,null,2]
    /// - Output: false
    ///
    /// Example 3:
    /// ----------
    /// - Input: p = [1,2,1], q = [1,1,2]
    /// - Output: false
    ///
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in both trees is in the range [0, 100].
    /// -104 <= Node.val <= 104
    ///
    fn is_same_tree(p: Option<NodeRef>, q: Option<NodeRef>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }

        let (p, q) = (p.clone().unwrap(), q.clone().unwrap());
        let (pref, qref) = (p.borrow(), q.borrow());

        pref.val == qref.val &&
        Solution::is_same_tree(pref.left.clone(), qref.left.clone()) && 
        Solution::is_same_tree(pref.right.clone(), qref.right.clone())
    }

    /// ## 236. Lowest Common Ancestor of a Binary Tree
    ///
    /// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
    ///
    /// According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between 
    /// two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a 
    /// node to be a descendant of itself)."
    ///
    /// Example 1:
    /// ----------
    /// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
    /// 
    /// Output: 3
    /// 
    /// Explanation: The LCA of nodes 5 and 1 is 3.
    ///
    /// Example 2:
    /// ----------
    /// Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
    /// 
    /// Output: 5
    /// 
    /// Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according 
    /// to the LCA definition.
    ///
    /// Example 3:
    /// ----------
    /// Input: root = [1,2], p = 1, q = 2
    /// 
    /// Output: 1
    /// 
    fn lowest_common_ancestor(
        root: Option<NodeRef>, p: Option<NodeRef>,  q: Option<NodeRef>
    ) -> Option<NodeRef> {

        match root {
            Some(node) => {
                let n_borrow = node.borrow();
                if n_borrow.val == p.as_ref().unwrap().borrow().val || n_borrow.val == q.as_ref().unwrap().borrow().val {
                    return Some(node.clone());
                }
                let left = Solution::lowest_common_ancestor(n_borrow.left.clone(), p.clone(), q.clone());
                let right = Solution::lowest_common_ancestor(n_borrow.right.clone(), p.clone(), q.clone());
                if left.is_some() && right.is_some() {
                    return Some(node.clone());
                }
                left.or(right)
            },
            None => return None
        }    
    }

    /// ## 212. Word Search II
    /// https://leetcode.com/problems/word-search-ii/
    ///
    /// Given an m x n board of characters and a list of strings words, return all words on the board.
    ///
    /// Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells 
    /// are horizontally or vertically neighboring. The same letter cell may not be used more than once 
    /// in a word.
    ///
    /// Example 1:
    /// ----------
    /// - Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], 
    /// words = ["oath","pea","eat","rain"]
    /// -  Output: ["eat","oath"]
    ///
    /// Example 2:
    /// ----------
    /// - Input: board = [["a","b"],["c","d"]], words = ["abcb"]
    /// - Output: []
    ///
    ///
    /// Constraints:
    /// ------------
    /// m == board.length
    /// n == board[i].length
    /// 1 <= m, n <= 12
    /// board[i][j] is a lowercase English letter.
    /// 1 <= words.length <= 3 * 104
    /// 1 <= words[i].length <= 10
    /// words[i] consists of lowercase English letters.
    /// All the strings of words are unique.    
    ///
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        unimplemented!()
    }

    /// ## 30. Kth Smallest Element in a BST
    /// https://leetcode.com/problems/kth-smallest-element-in-a-bst/
    ///  
    /// Given the root of a binary search tree, and an integer k, return the kth smallest value 
    /// (1-indexed) of all the values of the nodes in the tree.
    ///
    /// Example 1:
    /// ----------
    /// - Input: root = [3,1,4,null,2], k = 1
    /// - Output: 1
    ///
    /// Example 2:
    /// ----------
    /// - Input: root = [5,3,6,2,4,null,null,1], k = 3
    /// - Output: 3
    ///
    /// Constraints:
    /// --------------
    /// The number of nodes in the tree is n.
    /// 1 <= k <= n <= 104
    /// 0 <= Node.val <= 104
    ///
    pub fn kth_smallest(root: NodeOption, k: i32) -> i32 {
        unimplemented!()
    }

    /// ## 105. Construct Binary Tree from Preorder and Inorder Traversal
    /// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
    ///
    /// Given two integer arrays preorder and inorder where preorder is the preorder traversal of 
    /// a binary tree and inorder is the inorder traversal of the same tree, construct and return 
    /// the binary tree.
    ///
    ///
    /// Example 1:
    /// ----------
    /// - Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
    /// - Output: [3,9,20,null,null,15,7]
    ///
    /// Example 2:
    /// ----------
    /// - Input: preorder = [-1], inorder = [-1]
    /// - Output: [-1]
    ///
    /// Constraints:
    /// ------------
    /// - 1 <= preorder.length <= 3000
    /// - inorder.length == preorder.length
    /// - -3000 <= preorder[i], inorder[i] <= 3000
    /// - preorder and inorder consist of unique values.
    /// - Each value of inorder also appears in preorder.
    /// - preorder is guaranteed to be the preorder traversal of the tree.
    /// - inorder is guaranteed to be the inorder traversal of the tree.
    ///
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> NodeOption {
        unimplemented!()
    }


    /// ## 124. Binary Tree Maximum Path Sum
    /// https://leetcode.com/problems/binary-tree-maximum-path-sum/
    ///
    /// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in 
    /// the sequence has an edge connecting them. A node can only appear in the sequence at 
    /// most once. Note that the path does not need to pass through the root.
    ///
    /// The path sum of a path is the sum of the node's values in the path.
    ///
    /// Given the root of a binary tree, return the maximum path sum of any non-empty path.
    ///
    ///
    /// Example 1:
    /// ------------
    /// - Input: root = [1,2,3]
    /// - Output: 6
    /// - Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
    ///
    /// Example 2:
    /// ------------
    /// - Input: root = [-10,9,20,null,null,15,7]
    /// - Output: 42
    /// - Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
    ///
    /// Constraints:
    /// ------------
    /// - The number of nodes in the tree is in the range [1, 3 * 104].
    /// - -1000 <= Node.val <= 1000
    ///
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        unimplemented!()
    }

    /// ## 572. Subtree of Another Tree
    ///
    ///
    ///
    /// Given the roots of two binary trees root and subRoot, return true if there is a subtree of 
    /// root with the same structure and node values of subRoot and false otherwise.
    ///
    /// A subtree of a binary tree tree is a tree that consists of a node in tree and all of this 
    /// node's descendants. The tree tree could also be considered as a subtree of itself.
    ///
    ///
    /// Example 1:
    /// -----------
    /// Input: root = [3,4,5,1,2], subRoot = [4,1,2]
    /// Output: true
    ///
    /// Example 2:
    /// ------------
    /// Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
    /// Output: false
    ///
    /// Constraints:
    /// --------------
    /// The number of nodes in the root tree is in the range [1, 2000].
    /// The number of nodes in the subRoot tree is in the range [1, 1000].
    /// -104 <= root.val <= 104
    /// -104 <= subRoot.val <= 104
    ///
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        unimplemented!()
    }

    
    
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth(None), 0);
    }

    #[test]
    fn test_lowest_common_ancestor() {
        // Test case 1
        let root = tree_node!(
            3,
            tree_node!(
                5, 
                tree_node!(6), 
                tree_node!(
                    2,
                    tree_node!(7), tree_node!(4)
                )
            ),
            tree_node!(
                1, tree_node!(0), tree_node!(8)
            )
        );

        let (p, q) = (tree_node!(5), tree_node!(1));
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(3));

        // Test case 2
        let (p, q) = (tree_node!(5), tree_node!(4));
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(5));

        // Test case 3
        let root = tree_node!(
            1, tree_node!(2), None
        );
        let (p, q) = (tree_node!(1), tree_node!(2));
        assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(1));

    }
}