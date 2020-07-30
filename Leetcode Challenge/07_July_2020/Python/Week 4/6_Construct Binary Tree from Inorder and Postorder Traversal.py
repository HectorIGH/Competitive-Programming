#Given inorder and postorder traversal of a tree, construct the binary tree.
#
#Note:
#You may assume that duplicates do not exist in the tree.
#
#For example, given
#
#inorder = [9,3,15,20,7]
#postorder = [9,15,7,20,3]
#Return the following binary tree:
#
#    3
#   / \
#  9  20
#    /  \
#   15   7

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        self.index = {}
        il = len(inorder)
        for i in range(il):
            self.index[inorder[i]] = i
        return self.aux(inorder, postorder, 0, il - 1, len(postorder) - 1)
    def aux(self, inorder, postorder, start, end, index):
        if start > end:
            return None
        root = TreeNode(postorder[index])
        rootIndex = self.index[root.val]
        
        root.right = self.aux(inorder, postorder, rootIndex + 1, end, index - 1)
        root.left = self.aux(inorder, postorder, start, rootIndex - 1, index - (end - rootIndex) - 1)
        
        return root