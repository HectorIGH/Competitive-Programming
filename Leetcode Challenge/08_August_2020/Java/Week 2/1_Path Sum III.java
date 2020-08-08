//You are given a binary tree in which each node contains an integer value.
//
//Find the number of paths that sum to a given value.
//
//The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
//
//The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
//
//Example:
//
//root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
//
//      10
//     /  \
//    5   -3
//   / \    \
//  3   2   11
// / \   \
//3  -2   1
//
//Return 3. The paths that sum to 8 are:
//
//1.  5 -> 3
//2.  5 -> 2 -> 1
//3. -3 -> 11

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    int result = 0;
    HashMap<Integer, Integer> count = new HashMap<>();
    public int pathSum(TreeNode root, int sum) {
        /*
        if(root == null) {
            return 0;
        } else {
            return dfs(root, sum) + pathSum(root.left, sum) + pathSum(root.right, sum);
        }
        */
        count.put(sum, 1);
        dfs(root, sum , 0);
        return result;
    }
    /*
    public int dfs(TreeNode root, int sum) {
        if(root == null) {
            return 0;
        } else {
            int ans = root.val == sum ? 1 : 0;
            return ans + dfs(root.left, sum - root.val) + dfs(root.right, sum - root.val);
        }
    }
    */
    public void dfs(TreeNode root, int sum, int root_sum) {
        if(root == null) {
            return ;
        } else {
            root_sum += root.val;
            result += count.getOrDefault(root_sum, 0);
            
            int cur = count.getOrDefault(root_sum + sum, 0);
            count.put(root_sum + sum, cur + 1);
            
            dfs(root.left, sum, root_sum);
            dfs(root.right, sum, root_sum);
            
            cur = count.getOrDefault(root_sum + sum, 0);
            count.put(root_sum + sum, cur - 1);
        }
    }
}