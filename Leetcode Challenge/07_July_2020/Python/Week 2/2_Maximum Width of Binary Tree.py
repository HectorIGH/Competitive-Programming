#Given a binary tree, write a function to get the maximum width of the given tree. The maximum width of a tree is the maximum width among all levels.
#
#The width of one level is defined as the length between the end-nodes (the leftmost and right most non-null nodes in the level, where the null nodes between the end-nodes are also counted into the length calculation.
#
#It is guaranteed that the answer will in the range of 32-bit signed integer.
#
#Example 1:
#
#Input: 
#
#           1
#         /   \
#        3     2
#       / \     \  
#      5   3     9 
#
#Output: 4
#Explanation: The maximum width existing in the third level with the length 4 (5,3,null,9).
#Example 2:
#
#Input: 
#
#          1
#         /  
#        3    
#       / \       
#      5   3     
#
#Output: 2
#Explanation: The maximum width existing in the third level with the length 2 (5,3).
#Example 3:
#
#Input: 
#
#          1
#         / \
#        3   2 
#       /        
#      5      
#
#Output: 2
#Explanation: The maximum width existing in the second level with the length 2 (3,2).
#Example 4:
#
#Input: 
#
#          1
#         / \
#        3   2
#       /     \  
#      5       9 
#     /         \
#    6           7
#Output: 8
#Explanation:The maximum width existing in the fourth level with the length 8 (6,null,null,null,null,null,null,7).
# 
#
#Constraints:
#
#The given binary tree will have between 1 and 3000 nodes.

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def widthOfBinaryTree(self, root: TreeNode) -> int:
        '''
        # BFS
        width = 0
        q = deque()
        q.append((root, 1))
        while q:
            n = len(q)
            width = max(width, q[-1][1] - q[0][1] + 1)
            while n:
                node, pos = q.popleft()
                if node.left:
                    q.append((node.left, pos * 2))
                if node.right:
                    q.append((node.right, pos * 2 + 1))
                n -= 1
        return width
        '''
        # DFS
        self.levelLMN = []
        def dfs(root, idx, level, levelLMN):
            if not root:
                return 0
            if(level == len(levelLMN)):
                levelLMN.append(idx)
            return max(idx + 1 - levelLMN[level],
                       dfs(root.left, idx * 2, level + 1, levelLMN),
                       dfs(root.right, idx * 2 + 1, level + 1, levelLMN))
        return dfs(root, 1, 0, self.levelLMN)
        