/* 
297. Serialize and Deserialize Binary Tree
https://leetcode.com/problems/serialize-and-deserialize-binary-tree/description/

Serialization is the process of converting a data structure or object into a sequence 
of bits so that it can be stored in a file or memory buffer, or transmitted across a 
network connection link to be reconstructed later in the same or another computer 
environment.

Design an algorithm to serialize and deserialize a binary tree. There is no restriction 
on how your serialization/deserialization algorithm should work. You just need to ensure 
that a binary tree can be serialized to a string and this string can be deserialized to 
the original tree structure.

Clarification: The input/output format is the same as how LeetCode serializes a binary 
tree. You do not necessarily need to follow this format, so please be creative and come 
up with different approaches yourself.


Example 1:
----------
Input: root = [1,2,3,null,null,4,5]
Output: [1,2,3,null,null,4,5]

Example 2:
----------
Input: root = []
Output: []

Constraints:
----------
The number of nodes in the tree is in the range [0, 104].
-1000 <= Node.val <= 1000

 */

use std::cell::{RefCell, Ref};
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

trait TreeBuilder {
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(
            TreeNode {val, left: None, right: None}
        )))
    }

    fn node(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(
            TreeNode {val, left, right}
        )))
    }
}

impl TreeBuilder for TreeLink {}


struct Solution;


impl Solution {
    /// Output: 1, 2, null, null, 3, 4, null, null, 5, null, null
    fn serialize_preorder(root: &TreeLink, ans: &mut Vec<i32>) {
        match root {
            Some(node) => {
                let node = node.borrow();
                let val = node.val;
                ans.push(val);
                Solution::serialize_preorder(&node.left, ans);
                Solution::serialize_preorder(&node.right, ans);
            }
            None => ans.push(-1),
        }
    }

    /// Encodes a tree to a single string
    /// Serialize tree with pre-order traversal
    #[allow(dead_code)]
    fn serialize(root: TreeLink) -> String {
        
        let mut ans: Vec<i32> = vec![];
        Solution::serialize_preorder(&root, &mut ans);
        
        let serialize_str = format!("{:?}", ans);
        serialize_str
    }

    /// De-Serialize the pre-order list to tree node
    fn deserialize_preorder(data: &mut Vec<&str>, ans: &mut TreeLink) {
        /* 
        // implementation of index_of in a Vec
        // https://stackoverflow.com/questions/26243025/how-to-remove-an-element-from-a-vector-given-the-element
        let index = xs.iter().position(|x| *x == some_x).unwrap();
        xs.remove(index);
        
         */
        if data[0] == "null" {
            data.remove(0);
            return;
        }
        let x = data.remove(0).parse::<i32>().unwrap();
        *ans = TreeLink::leaf(x);
        if let Some(refnode) = ans {
            let mut node = refnode.borrow_mut();
            Solution::deserialize_preorder(data, &mut node.left);
            Solution::deserialize_preorder(data, &mut node.right);
        }

    }

    /// Decodes your encoded data to tree.
    #[allow(dead_code)]
    fn deserialize(data: String) -> TreeLink {
    
        let mut data: Vec<&str> = data.split(',').collect();
        let mut ans: TreeLink = None;
        Solution::deserialize_preorder(&mut data, &mut ans);
        ans
    }
    
         

}


#[test]
fn test() {
    //  ser = Codec()
    //  deser = Codec()
    //  root = None
    //  ans = deser.deserialize(ser.serialize(root))
}

