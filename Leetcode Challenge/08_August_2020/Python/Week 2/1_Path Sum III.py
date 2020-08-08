#You are given a binary tree in which each node contains an integer value.
#
#Find the number of paths that sum to a given value.
#
#The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
#
#The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
#
#Example:
#
#root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
#
#      10
#     /  \
#    5   -3
#   / \    \
#  3   2   11
# / \   \
#3  -2   1
#
#Return 3. The paths that sum to 8 are:
#
#1.  5 -> 3
#2.  5 -> 2 -> 1
#3. -3 -> 11

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pathSum(self, root: TreeNode, sum: int) -> int:
        '''
        if not root:
            return 0
        else:
            return self.dfs(root, sum) + self.pathSum(root.left, sum) + self.pathSum(root.right, sum)
    
    def dfs(self, root: TreeNode, sum: int):
        if not root:
            return 0
        else:
            ans = root.val == sum
            return ans + self.dfs(root.left, sum - root.val) + self.dfs(root.right, sum - root.val)
        '''
        self.result = 0
        self.count = defaultdict(int)
        self.count[sum] = 1
        self.dfs(root, sum , 0)
        return self.result
    
    def dfs(self, root, sum, root_sum):
        if not root:
            return 0
        root_sum += root.val
        self.result += self.count[root_sum]
        self.count[root_sum + sum] += 1
        self.dfs(root.left, sum, root_sum)
        self.dfs(root.right, sum, root_sum)
        self.count[root_sum + sum] -= 1