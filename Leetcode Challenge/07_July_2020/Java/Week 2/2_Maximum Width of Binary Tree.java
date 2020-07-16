//Given a binary tree, write a function to get the maximum width of the given tree. The maximum width of a tree is the maximum width among all levels.
//
//The width of one level is defined as the length between the end-nodes (the leftmost and right most non-null nodes in the level, where the null nodes between the end-nodes are also counted into the length calculation.
//
//It is guaranteed that the answer will in the range of 32-bit signed integer.
//
//Example 1:
//
//Input: 
//
//           1
//         /   \
//        3     2
//       / \     \  
//      5   3     9 
//
//Output: 4
//Explanation: The maximum width existing in the third level with the length 4 (5,3,null,9).
//Example 2:
//
//Input: 
//
//          1
//         /  
//        3    
//       / \       
//      5   3     
//
//Output: 2
//Explanation: The maximum width existing in the third level with the length 2 (5,3).
//Example 3:
//
//Input: 
//
//          1
//         / \
//        3   2 
//       /        
//      5      
//
//Output: 2
//Explanation: The maximum width existing in the second level with the length 2 (3,2).
//Example 4:
//
//Input: 
//
//          1
//         / \
//        3   2
//       /     \  
//      5       9 
//     /         \
//    6           7
//Output: 8
//Explanation:The maximum width existing in the fourth level with the length 8 (6,null,null,null,null,null,null,7).
// 
//
//Constraints:
//
//The given binary tree will have between 1 and 3000 nodes.

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
    public int widthOfBinaryTree(TreeNode root) {
        
        // BFS
        int n = 0, width = 0;
        Deque<Pair<TreeNode, Integer>> q = new LinkedList<>();
        q.add(new Pair(root, 1));
        while(!q.isEmpty()) {
            n = q.size();
            width = Math.max(width, q.peekLast().getValue() - q.peekFirst().getValue() + 1);
            while(n > 0) {
                Pair<TreeNode, Integer> node = q.poll();
                if(node.getKey().left != null) {
                    q.add(new Pair(node.getKey().left, 2 * node.getValue()));
                }
                if(node.getKey().right != null) {
                    q.add(new Pair(node.getKey().right, 2 * node.getValue() + 1));
                }
                n--;
            }
        }
        return width;
        
        /*
        // DFS
        List<Integer> levelLMN = new ArrayList<>();
        return dfs(root, 1, 0, levelLMN);
        */
    }
    /*
    int dfs(TreeNode root, int id, int level, List<Integer> levelLMN) {
        if(root == null) {
            return 0;
        }
        if(level == levelLMN.size()) {
            levelLMN.add(id);
        }
        return Math.max(id + 1 - levelLMN.get(level),
                        Math.max(dfs(root.left, id * 2, level + 1, levelLMN), dfs(root.right,id * 2 + 1, level + 1, levelLMN)));
    }
    */
}