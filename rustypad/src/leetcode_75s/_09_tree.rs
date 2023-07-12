#![allow(unused)]
use std::collections::HashMap;
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

        _iterative_max_depth(&root)
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
    pub fn invert_tree(root: Option<NodeRef>) -> Option<NodeRef> {
        match root {
            None => root,

            Some(node) => {
                let mut node_borrow = node.borrow_mut();
                let left: Option<NodeRef> = Solution::invert_tree(node_borrow.left.take());
                let right: Option<NodeRef> = Solution::invert_tree(node_borrow.right.take());
                node_borrow.left = right;
                node_borrow.right = left;
                Some(node.clone())
            }
        }
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

        if let (Some(p), Some(q)) = (p, q) {
            let (pref, qref) = (p.borrow(), q.borrow());
    
            pref.val == qref.val &&
            Solution::is_same_tree(pref.left.clone(), qref.left.clone()) && 
            Solution::is_same_tree(pref.right.clone(), qref.right.clone())

        } else {
            false
        }
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
    /// ```
    /// let root = tree_node!(
    ///     3,
    ///     tree_node!(
    ///         5, 
    ///         tree_node!(6), 
    ///         tree_node!(2, tree_node!(7), tree_node!(4))
    ///     ),
    ///     tree_node!(1, tree_node!(0), tree_node!(8))
    /// );
    /// let (p, q) = (tree_node!(5), tree_node!(1));
    /// assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(3));
    /// ```
    /// **Explanation**: The LCA of nodes 5 and 1 is 3.
    ///
    /// Example 2:
    /// ----------
    /// ```
    /// let (p, q) = (tree_node!(5), tree_node!(4));
    /// assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(5));
    /// ```
    /// **Explanation**: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according 
    /// to the LCA definition.
    ///
    /// Example 3:
    /// ----------
    /// ```
    /// let root = tree_node!(1, tree_node!(2), None);
    /// let (p, q) = (tree_node!(1), tree_node!(2));
    /// assert_eq!(Solution::lowest_common_ancestor(root.clone(), p, q), tree_node!(1));
    /// ```
    /// 
    fn lowest_common_ancestor(
        root: Option<NodeRef>, p: Option<NodeRef>,  q: Option<NodeRef>
    ) -> Option<NodeRef> {

        match root {
            Some(node) => {
                let n_borrow = node.borrow();
                // If the current node is either p or q, return the node
                if n_borrow.val == p.as_ref().unwrap().borrow().val || n_borrow.val == q.as_ref().unwrap().borrow().val {
                    return Some(node.clone());
                }
                let left: Option<NodeRef> = Solution::lowest_common_ancestor(n_borrow.left.clone(), p.clone(), q.clone());
                let right: Option<NodeRef> = Solution::lowest_common_ancestor(n_borrow.right.clone(), p.clone(), q.clone());
                // If both the left and right node has been found, return the node
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
    /// ```
    /// let board = vec![
    ///     vec!["o","a","a","n"], 
    ///     vec!["e","t","a","e"],
    ///     vec!["i","h","k","r"],
    ///     vec!["i","f","l","v"]], 
    /// let words = vec!["oath","pea","eat","rain"]
    /// assert_eq!(Solution::find_words(board, words), vec!["eat","oath"])
    /// 
    /// ```
    /// Example 2:
    /// ----------
    /// ```
    /// let board = vec![vec!["a","b"],vec!["c","d"]]; let words = vec!["abcb"];
    /// assert_eq!(Solution::find_words(board, words), vec![])
    /// ```
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
        // fn inorder_traversal(root: Option<NodeRef>) -> Vec<i32> {
        //     if root.is_none() {
        //         return vec![]
        //     }
        //     let mut ans: Vec<i32> = vec![];
        //     if let Some(node) = root{
        //         ans.extend(inorder_traversal(node.borrow().left.clone()));
        //         ans.push(node.borrow().val);
        //         ans.extend(inorder_traversal(node.borrow().right.clone()));
        //     }
        //     ans
        // }

        // let ans = inorder_traversal(root);
        // *ans.iter().nth(k as usize).unwrap()

        fn inorder_traversal(root: Option<NodeRef>, k: &mut i32) -> Option<NodeRef> {
            match root {
                Some(node) => {
                    let left: Option<NodeRef> = inorder_traversal(node.borrow().left.clone(), k);
                    if left .is_some() {
                        return left;
                    }
                    *k -= 1;
                    if *k == 0 {
                        return Some(node);    
                    }
                    let right: Option<NodeRef> = inorder_traversal(node.borrow().right.clone(), k);
                    right
                },
                None => {
                    None
                }
            }
        }
        let mut k = k;
        let ans: Option<NodeRef> = inorder_traversal(root, &mut k);
        ans.unwrap().borrow().val
    
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
        fn tree_from_preorder(
            start: i32, end: i32, 
            pre_index: &mut usize,
            preorder: &Vec<i32>,
            inorder_map: &HashMap<i32, i32>
        ) -> Option<NodeRef> {
            // Start and end are inorder index
            if start > end {
                return None;
            }

            let val = preorder[*pre_index];
            let mut root: NodeRef = Rc::new(RefCell::new(TreeNode {val, left: None, right: None}));
            *pre_index += 1;
            
            root.borrow_mut().left = tree_from_preorder(
                start, inorder_map[&val] - 1, pre_index, preorder, inorder_map
            );
            root.borrow_mut().right = tree_from_preorder(
                inorder_map[&val] + 1, end, pre_index, preorder, inorder_map
            );
            Some(root)
        }

        let mut pre_index = 0;
        let inorder_map: HashMap<i32, i32> = 
            inorder
            .iter().enumerate()
            .map(|(i, v)| (*v, i as i32)).collect();

        let (start, end) = (0, (preorder.len() - 1) as i32);
        let tree: Option<NodeRef> = tree_from_preorder(start, end, &mut pre_index, &preorder, &inorder_map);
        tree
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
    fn is_identical(root: Option<NodeRef>, sub_root: Option<NodeRef>) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(node), Some(sub_node)) => {
                let subnode_borrow = sub_node.borrow();
                let node_borrow = node.borrow();
                node_borrow.val == subnode_borrow.val &&
                Solution::is_identical(node_borrow.left.clone(), subnode_borrow.left.clone()) &&
                Solution::is_identical(node_borrow.right.clone(), subnode_borrow.right.clone())
            },
            _ => false
        }
    }

    pub fn is_subtree(root: Option<NodeRef>, sub_root: Option<NodeRef>) -> bool {
        match (root) {
            None => false,
            Some(node) => {
                if Solution::is_identical(Some(node.clone()), sub_root.clone()) {
                    return true;
                }
                Solution::is_subtree(node.borrow().left.clone(), sub_root.clone()) ||
                Solution::is_subtree(node.borrow().right.clone(), sub_root.clone())
            }
        }
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