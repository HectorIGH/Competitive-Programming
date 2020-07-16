//Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
//
//For example:
//Given binary tree [3,9,20,null,null,15,7],
//    3
//   / \
//  9  20
//    /  \
//   15   7
//return its bottom-up level order traversal as:
//[
//  [15,7],
//  [9,20],
//  [3]
//]

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
    List<List<Integer>> ans = new ArrayList<>(); // for dfs
    public List<List<Integer>> levelOrderBottom(TreeNode root) {
        
        // BFS
        if(root == null) {
            return new ArrayList();
        }
        List<List<Integer>> ans = new ArrayList<>();
        Queue<TreeNode> dq = new LinkedList<>();
        dq.add(root);
        dq.add(null);
        while(dq.peek() != null) {
            List<Integer> holder = new ArrayList();
            int n = dq.size();
            while(dq.peek() != null) {
                TreeNode node = dq.poll();
                holder.add(node.val);
                if(node.left != null) {
                    dq.add(node.left);
                }
                if(node.right != null) {
                    dq.add(node.right);
                }
            }
            ans.add(holder);
            dq.poll();
            dq.add(null);
        }
        Collections.reverse(ans);
        return ans;
        /*
        // DFS
        if(root == null) {
            return new ArrayList();
        }
        dfs(root, 0);
        Collections.reverse(ans);
        return ans;
        */
    }
    /*
    void dfs(TreeNode root, Integer level) {
        if(root == null) {
            return;
        }
        if(level == ans.size()) {
            ans.add(new ArrayList());
        }
        ans.get(level).add(root.val);
        dfs(root.left, level + 1);
        dfs(root.right, level + 1);
    }
    */
}