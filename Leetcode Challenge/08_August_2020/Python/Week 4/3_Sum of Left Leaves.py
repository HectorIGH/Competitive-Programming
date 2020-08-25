#Find the sum of all left leaves in a given binary tree.
#
#Example:
#
#    3
#   / \
#  9  20
#    /  \
#   15   7
#
#There are two left leaves in the binary tree, with values 9 and 15 respectively. Return 24.

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sumOfLeftLeaves(self, root: TreeNode) -> int:
        def aux(root, l):
            if not root:
                return 0
            else:
                if not root.left and not root.right:
                    return root.val * l
                else:
                    return aux(root.left, 1) + aux(root.right, 0)
        return aux(root, 0)