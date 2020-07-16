#Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
#
#For example:
#Given binary tree [3,9,20,null,null,15,7],
#    3
#   / \
#  9  20
#    /  \
#   15   7
#return its bottom-up level order traversal as:
#[
#  [15,7],
#  [9,20],
#  [3]
#]

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def levelOrderBottom(self, root: TreeNode) -> List[List[int]]:
        '''
        # BFS
        if not root:
            return []
        ans, dq = [], deque([root])
        while dq:
            n = len(dq)
            holder = []
            for i in range(n):
                node = dq.popleft()
                holder.append(node.val)
                if(node.left):
                    dq.append(node.left)
                if(node.right):
                    dq.append(node.right)
            ans.append(holder)
            
        return ans[::-1]
        '''
        # DFS
        if not root:
            return []
        self.ans = []
        self.dfs(root, 0)
        return self.ans[::-1]
    
    def dfs(self, root, level):
        if not root:
            return
        if level == len(self.ans):
            self.ans.append([])
        self.ans[level].append(root.val)
        self.dfs(root.left, level + 1)
        self.dfs(root.right, level + 1)