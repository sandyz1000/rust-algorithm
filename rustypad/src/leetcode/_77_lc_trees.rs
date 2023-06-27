#![allow(unused)]

use std::cmp::min;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;


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

#[derive(Debug, PartialEq, Eq)]
struct Solution;


impl Solution {
    
    /// ## 112. Path Sum
    ///
    /// Given the root of a binary tree and an integer targetSum, return true if the tree has a 
    /// root-to-leaf path such that adding up all the values along the path equals targetSum.
    ///
    /// A leaf is a node with no children.
    ///
    /// Example 1:
    /// ----------  
    /// Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
    /// Output: true
    /// Explanation: The root-to-leaf path with the target sum is shown.
    ///
    /// Example 2:
    /// ---------
    /// Input: root = [1,2,3], targetSum = 5
    /// Output: false
    /// Explanation: There two root-to-leaf paths in the tree:
    /// (1 --> 2): The sum is 3.
    /// (1 --> 3): The sum is 4.
    /// There is no root-to-leaf path with sum = 5.
    /// 
    /// Example 3:
    /// ---------
    /// Input: root = [], targetSum = 0
    /// Output: false
    /// Explanation: Since the tree is empty, there are no root-to-leaf paths.
    ///
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in the tree is in the range [0, 5000].
    /// -1000 <= Node.val <= 1000
    /// -1000 <= targetSum <= 1000
    fn has_path_sum(root: Option<NodeRef>, target_sum: i32) -> bool {
        fn _path_sum_recurse(
            root: &Option<NodeRef>, 
            target_sum: i32, 
            mut sum: i32
        ) -> bool {
            if root.is_none() {
                return false;
            }
            // Convert reference to option to internal reference
            let root = root.as_ref().unwrap();
            let noderef = root.borrow();

            // If current node is leaf node
            if noderef.left.is_none() && noderef.right.is_none() {
                let current_sum: i32 = (noderef.val + sum);
                return current_sum == target_sum;
            }

            sum += noderef.val;
            let left = _path_sum_recurse(
                &noderef.left, target_sum, sum
            );
            let right = _path_sum_recurse(
                &noderef.right, target_sum, sum
            );

            left || right
        }


        _path_sum_recurse(&root, target_sum, 0)
    }

    /// ## 1448. Count Good Nodes in Binary Tree
    ///
    /// Given a binary tree root, a node X in the tree is named good if in the path from root to X 
    /// there are no nodes with a value greater than X.
    ///
    /// Return the number of good nodes in the binary tree.
    ///
    /// Example 1:
    ///
    /// Input: root = [3,1,4,3,null,1,5]
    /// Output: 4
    /// Explanation: Nodes in blue are good.
    /// Root Node (3) is always a good node.
    /// Node 4 -> (3,4) is the maximum value in the path starting from the root.
    /// Node 5 -> (3,4,5) is the maximum value in the path
    /// Node 3 -> (3,1,3) is the maximum value in the path.
    ///
    /// Example 2:
    ///
    /// Input: root = [3,3,null,4,2]
    /// Output: 3
    /// Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.
    ///
    /// Example 3:
    ///
    /// Input: root = [1]
    /// Output: 1
    /// Explanation: Root is considered as good.
    ///
    /// Constraints:
    ///
    /// The number of nodes in the binary tree is in the range [1, 10^5].
    /// Each node's value is between [-10^4, 10^4].
    ///
    fn good_nodes(root: Option<NodeRef>) -> i32 {
        // Pre-order traversal to find good nodes with current max value
        fn _pre_order(root: Option<NodeRef>, mut current_max: i32) -> i32 {
            match root {
                Some(node) => {
                    let n_borrow = node.borrow();
                    let left = _pre_order(n_borrow.left.clone(), current_max.max(n_borrow.val));
                    let right = _pre_order(n_borrow.right.clone(), current_max.max(n_borrow.val));
                    let mut ans = left + right;
                    if n_borrow.val > current_max {
                        ans += 1; 
                    }
                    ans
                },
                None => return 0
            }
        }
        
        _pre_order(root, i32::MIN)
    }

    

    fn min_depth(root: Option<NodeRef>) -> i32 {
        if root.is_none() {
            return 0;
        }
        
        let root = root.unwrap();
        let noderef = root.borrow();
        
        // If only one of child is non-null, then go into that recursion.
        if noderef.left.is_none() {
            return 1 + Solution::min_depth(noderef.right.clone());
        } else if noderef.right.is_none() {
            return 1 + Solution::min_depth(noderef.left.clone());
        }

        let left_min = Solution::min_depth(noderef.left.clone());
        let right_min = Solution::min_depth(noderef.right.clone());
        1 + std::cmp::min(left_min, right_min)
    }

    /// ## 1026. Maximum Difference Between Node and Ancestor
    /// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/description/
    ///
    /// Given the root of a binary tree, find the maximum value v for which there exist different nodes 
    /// a and b where v = |a.val - b.val| and a is an ancestor of b.
    ///
    /// A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.
    ///
    ///
    /// Example 1:
    /// ----------
    ///
    /// Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
    /// Output: 7
    /// Explanation: We have various ancestor-node differences, some of which are given below :
    /// |8 - 3| = 5
    /// |3 - 7| = 4
    /// |8 - 1| = 7
    /// |10 - 13| = 3
    /// Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
    /// 
    /// Example 2:
    /// ----------
    /// Input: root = [1,null,2,null,0,3]
    /// Output: 3
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the tree is in the range [2, 5000].
    /// 0 <= Node.val <= 105
    /// 
    fn max_ancestor_diff(root: Option<NodeRef>) -> i32 {
        // Send the current max & min to the recursion function.
        // Base case: If node.is_none(), return 0.
        // When the leaf node is reached calculate abs diff
        if root.is_none() {
            return 0;
        }
        fn _max_ancestor_diff(
            root: Option<NodeRef>, 
            mut current_max: i32, 
            mut current_min: i32
        ) -> i32 {
            // When leaf node is reached calculate the diff
            if root.is_none() {
                return (current_max - current_min).abs();
            }

            // Calculate the max_diff here
            let root = root.unwrap();
            let noderef = root.borrow();

            current_max = current_max.max(noderef.val);
            current_min = current_min.min(noderef.val);
            
            let left_max_diff = _max_ancestor_diff(noderef.left.clone(), current_max, current_min);
            let right_max_diff = _max_ancestor_diff(noderef.right.clone(), current_max, current_min);
            
            std::cmp::max(right_max_diff, left_max_diff)
        }
        
        let node: &NodeRef = root.as_ref().unwrap();
        let mut val: i32 = node.borrow().val;
        let max_diff = _max_ancestor_diff(root, val, val);
        max_diff

    }


    fn diameter_of_binary_tree(root: Option<NodeRef>) -> i32 {
        fn depth(root: &Option<NodeRef>, max: &mut i32) -> i32 {
            if let Some(node) = root {
                let left = depth(&node.borrow().left, max);
                let right = depth(&node.borrow().right, max);
                
                // update the diameter if left_path plus right_path is larger
                *max = (*max).max(left + right);
                std::cmp::max(left, right) + 1
            } else {
                0
            }
        }

        let mut max_diameter = 0;
        depth(&root, &mut max_diameter);
        max_diameter
    }

    /// ## 199. Binary Tree Right Side View
    ///
    /// Given the root of a binary tree, imagine yourself standing on the right side of it, 
    /// return the values of the nodes you can see ordered from top to bottom.
    ///
    /// Example 1:
    /// ----------
    /// Input: root = [1,2,3,null,5,null,4]
    /// Output: [1,3,4]
    /// 
    /// Example 2:
    /// ----------
    /// Input: root = [1,null,3]
    /// Output: [1,3]
    /// 
    /// Example 3:
    /// ----------
    /// Input: root = []
    /// Output: []
    ///
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in the tree is in the range [0, 100].
    /// -100 <= Node.val <= 100
    /// 
    fn right_side_view(root: Option<NodeRef>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut deq: VecDeque<Option<NodeRef>> = VecDeque::new();
        deq.push_back(root);
        
        while !deq.is_empty() {
            let node_count: i32 = deq.len() as i32;
            for i in 0..node_count {
                let node: Option<NodeRef> = deq.pop_front().unwrap();
                
                if let Some(node) = node.as_ref() {
                    // If this is the last node in the tree, push the value.
                    if i == node_count - 1 {
                        ans.push(node.borrow().val);
                    }
    
                    if node.borrow().left.is_some() {
                        deq.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        deq.push_back(node.borrow().right.clone());
                    }
                }
                // let node: &NodeRef = node.as_ref().unwrap();
            }
        }
        ans
    }

    fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut deq: VecDeque<Option<NodeRef>> = VecDeque::new();
        deq.push_back(root);

        while !deq.is_empty() {
            let node_count: usize = deq.len();
            // Find max at each level
            let mut max_val: i32 = i32::MIN;

            for i in 0..node_count {
                let node: Option<NodeRef> = deq.pop_front().unwrap();
                if let Some(node) = node.as_ref() {
                    let noderef: Ref<TreeNode> = node.borrow();
                    
                    max_val = max_val.max(noderef.val);
                    
                    if noderef.left.is_some() {
                        deq.push_back(noderef.left.clone());
                    }

                    if noderef.right.is_some() {
                        deq.push_back(noderef.right.clone());
                    }
                }
            }

            ans.push(max_val);
        }

        ans   
    }

    fn deepest_leaves_sum(root: &Option<NodeRef>) -> i32 {
        fn max_depth(root: &Option<NodeRef>) -> i32 {
            
            if let Some(root) = root.as_ref() {
                let noderef = root.borrow();
                
                let left = max_depth(&noderef.left.clone());
                let right = max_depth(&noderef.right.clone());
                
                1 + left.max(right)
            } else {
                0
            }    
        }

        let tree_depth: i32 = max_depth(&root);
        let mut deq: VecDeque<(Option<NodeRef>, i32)> = VecDeque::new();
        let mut ans: Vec<i32> = vec![];
        
        deq.push_back((root.clone(), 1));
        
        // Traverse to last level and add to the ans
        while !deq.is_empty() {
            let node_count: i32 = deq.len() as i32;
            for i in 0..node_count {
                let (node, depth) = deq.pop_front().unwrap();
            
                if let Some(node) = node.as_ref() {
                    if depth == tree_depth {
                        ans.push(node.borrow().val);
                    }
                    
                    if node.borrow().left.is_some() {
                        deq.push_back((node.borrow().left.clone(), depth +1));
                    }

                    if node.borrow().right.is_some() {
                        deq.push_back((node.borrow().right.clone(), depth +1));
                    }
                }    
            }
        }
        
        ans.into_iter().sum()
    }

    fn zigzag_level_order(root: Option<NodeRef>) -> Vec<Vec<i32>> {
        
        let mut deq: VecDeque<(Option<NodeRef>, i32)> = VecDeque::new();
        let mut ans: Vec<Vec<i32>> = vec![];
        
        deq.push_back((root.clone(), 1));
        
        // Traverse to last level and add to the ans
        while !deq.is_empty() {
            let node_count: i32 = deq.len() as i32;
            let mut levels: Vec<i32> = vec![];
            let mut current_level: i32 = 0;
            for i in 0..node_count {
                let (node, level) = deq.pop_front().unwrap();
                
                // Update the current level here to reverse the order
                current_level = level;

                if let Some(node) = node.as_ref() {
                    levels.push(node.borrow().val);
                    
                    if node.borrow().left.is_some() {
                        deq.push_back((node.borrow().left.clone(), level + 1));
                    }

                    if node.borrow().right.is_some() {
                        deq.push_back((node.borrow().right.clone(), level + 1));
                    }
                }    
            }
            
            // For even levels
            if current_level % 2 == 0 {
                levels.reverse();
            }

            if !levels.is_empty() {
                ans.push(levels);
            }
        }

        ans
    }

    fn insert_into_bst(root: Option<NodeRef>, val: i32) -> Option<NodeRef> {
        fn insert_bst(root: Option<NodeRef>, val: i32) -> Option<NodeRef> {
            
            if let Some(r) = root.clone() {
                // Cannot create a variable that does mutable and immutable borrow 
                // in the same scope
                if r.borrow().val > val {
                    let left: Option<NodeRef> = insert_bst(r.borrow().left.clone(), val);
                    r.borrow_mut().left = left;
                } else {
                    let right: Option<NodeRef> = insert_bst(r.borrow().right.clone(), val);
                    r.borrow_mut().right = right;
                }
                root
            } else {
                let root: Option<NodeRef> = tree_node!(val);
                root
            }
            // unimplemented!();
        }
        
        insert_bst(root, val)
    }

    fn closest_value(root: NodeOption, target: f64) -> i32 {
        fn inorder_traversal(root: NodeOption, arr: &mut Vec<i32>) {
            if let Some(node) = root {
                inorder_traversal(node.borrow().left.clone(), arr);

                arr.push(node.borrow().val);
                inorder_traversal(node.borrow().right.clone(), arr);
            } 
        }

        let mut arr: Vec<i32> = vec![];
        inorder_traversal(root, &mut arr);
        let mut min: f64 = f64::MAX;
        let mut min_idx: usize = 0;

        for i in 0..arr.len() {
            let diff: f64 = (target - arr[i] as f64).abs();
            if diff < min {
                min = diff;
                min_idx = i;
            }
        }
        
        arr[min_idx]

    }


    fn closest_value_ii(root: NodeOption, target: f64) -> i32 {
        let mut closest = root.as_ref().unwrap().borrow().val;
        let mut root: NodeOption = root.clone();
        while let Some(r) = root {
            let diff1 = (r.borrow().val as f64 - target).abs();
            let diff2 = (closest as f64 - target).abs();
            closest = if diff1 < diff2 {r.borrow().val} else {closest};
            if r.borrow().val as f64 > target {
                root = r.borrow().left.clone();
            } else {
                root = r.borrow().right.clone();
            }
        }

        closest 
    }    

    /// ## 863. All Nodes Distance K in Binary Tree
    ///
    /// Given the root of a binary tree, the value of a target node target, and an integer k, 
    /// return an array of the values of all nodes that have a distance k from the target node.
    ///
    /// You can return the answer in any order.
    ///
    /// Example 1:
    /// ---------
    /// Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
    /// Output: [7,4,1]
    /// Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
    ///
    /// Example 2:
    /// ----------
    /// Input: root = [1], target = 1, k = 3
    /// Output: []
    ///
    /// Constraints:
    /// -----------
    /// The number of nodes in the tree is in the range [1, 500].
    /// 0 <= Node.val <= 500
    /// All the values Node.val are unique.
    /// target is the value of one of the nodes in the tree.
    /// 0 <= k <= 1000
    pub fn distance_k(root: Option<NodeRef>, target: Option<NodeRef>, k: i32) -> Vec<i32> {
        unimplemented!()   
    }

    /// ## 101. Symmetric Tree
    /// https://leetcode.com/problems/symmetric-tree/description/
    /// 
    /// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
    ///
    /// Example 1:
    /// ----------  
    /// - Input: root = [1,2,2,3,4,4,3]
    /// - Output: true
    ///
    /// Example 2:
    /// ----------
    /// - Input: root = [1,2,2,null,3,null,3]
    /// - Output: false
    ///
    /// Constraints:
    /// ------------
    /// The number of nodes in the tree is in the range [1, 1000].
    /// -100 <= Node.val <= 100
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        fn is_mirror(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (root1, root2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let n1_borrow = n1.borrow();
                    let n2_borrow = n2.borrow();
                    if n1_borrow.val != n2_borrow.val {
                        return false;
                    }

                    n1_borrow.val == n1_borrow.val &&
                    is_mirror(n1_borrow.left.clone(), n2_borrow.right.clone()) && 
                    is_mirror(n2_borrow.right.clone(), n1_borrow.left.clone())
                },
                _ => false
            }
        }
        // If the left side == right side, then the tree is symmetric
        is_mirror(root.clone(), root.clone())
    }

}


mod test {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        let root = tree_node!(
            5, 
            tree_node!(
                4, 
                tree_node!(11, tree_node!(7), tree_node!(2)), 
                None
            ), 
            tree_node!(
                8, 
                tree_node!(13), 
                tree_node!(4, None, tree_node!(1))
            )
        );

        assert_eq!(Solution::has_path_sum(root, 22), true);

        let root = tree_node!(1, tree_node!(2), tree_node!(3));
        assert_eq!(Solution::has_path_sum(root, 5), false);

        let root = None;
        assert_eq!(Solution::has_path_sum(root, 0), false);
    }

    #[test]
    fn test_min_depth() {
        unimplemented!()
    }

    #[test]
    fn test_right_side_view() {
        let root = tree_node!(
            1, 
            tree_node!(2, None, tree_node!(5)), 
            tree_node!(3, None, tree_node!(4))
        );

        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    #[test]
    fn test_largest_values() {
        let root = tree_node!(
            1, 
            tree_node!(3, tree_node!(5), tree_node!(3)), 
            tree_node!(2, None, tree_node!(9))
        );

        assert_eq!(Solution::largest_values(root), vec![1, 3, 9]);

        let root = tree_node!(1, tree_node!(2), tree_node!(3));
        assert_eq!(Solution::largest_values(root), vec![1, 3]);
    }

}
