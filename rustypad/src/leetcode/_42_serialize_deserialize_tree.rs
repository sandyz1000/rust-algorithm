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

from typing import List


class TreeNode(object):
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Codec:

    def serialize(self, root) -> str:
        """Encodes a tree to a single string.

        :type root: TreeNode
        :rtype: str
        """
        #  Serialize tree with pre-order traversal
        def _preorder(root: TreeNode, ans: List):
            """
            Output: 1, 2, null, null, 3, 4, null, null, 5, null, null
            """
            if root is None:
                ans.append(None)
                return

            ans.append(root.val)
            _preorder(root.left, ans)
            _preorder(root.right, ans)

        ans = []
        _preorder(root, ans)
        return ",".join([str(node) for node in ans])

    def deserialize(self, data: str) -> TreeNode:
        """Decodes your encoded data to tree.

        :type data: str
        :rtype: TreeNode
        """
        # De-Serialize the pre-order list to tree node
        def _preorder(data: List) -> TreeNode:
            if data[0] == "None":
                data.pop(0)
                return None
            node = TreeNode(int(data.pop(0)))
            node.left = _preorder(data)
            node.right = _preorder(data)
            return node

        data = data.split(',')
        root = _preorder(data)
        return root


def main():
    ser = Codec()
    deser = Codec()
    root = None
    ans = deser.deserialize(ser.serialize(root))
    

if __name__ == "__main__":
    main()
 */