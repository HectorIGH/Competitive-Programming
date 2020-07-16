#Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
#
#Example:
#Given a binary tree
#          1
#         / \
#        2   3
#       / \     
#      4   5    
#Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
#
#Note: The length of path between two nodes is represented by the number of edges between them.

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

## SOlution using array for locally storing ans. Seems slower
#def height(root: TreeNode, ans: List) -> int:
#        if root == None:
#            return 0
#        left_height = height(root.left, ans)
#        right_height = height(root.right, ans)
#        
#        ans[0] = max(ans[0], 0 + left_height + right_height) # 0 if the length is measure using edges, 1 if using #nodes
#        
#        return 1 + max(left_height, right_height)
#
#class Solution:
#        
#    def diameterOfBinaryTree(self, root: TreeNode) -> int:
#        if root == None:
#            return 0
#        ans = [-1e9 - 5]
#        
#        height_of_tree = height(root, ans)
#        
#        return ans[0]

# Solution using nonlocal variable for ans. Seems faster
class Solution:
        
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        def height(root: TreeNode) -> int:
            if root == None:
                return 0
            nonlocal ans
            
            left_height = height(root.left)
            right_height = height(root.right)

            ans = max(ans, 0 + left_height + right_height) # 0 if the length is measure using edges, 1 if using nodes

            return 1 + max(left_height, right_height)
        
        if root == None:
            return 0
        ans = -1e9 - 5
        
        height(root)
        
        return ans