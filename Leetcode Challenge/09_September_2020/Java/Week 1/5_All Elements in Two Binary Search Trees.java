//Given two binary search trees root1 and root2.
//
//Return a list containing all the integers from both trees sorted in ascending order.
//
// 
//
//Example 1:
//
//
//Input: root1 = [2,1,4], root2 = [1,0,3]
//Output: [0,1,1,2,3,4]
//Example 2:
//
//Input: root1 = [0,-10,10], root2 = [5,1,7,0,2]
//Output: [-10,0,0,1,2,5,7,10]
//Example 3:
//
//Input: root1 = [], root2 = [5,1,7,0,2]
//Output: [0,1,2,5,7]
//Example 4:
//
//Input: root1 = [0,-10,10], root2 = []
//Output: [-10,0,10]
//Example 5:
//
//
//Input: root1 = [1,null,8], root2 = [8,1]
//Output: [1,1,8,8]
// 
//
//Constraints:
//
//Each tree has at most 5000 nodes.
//Each node's value is between [-10^5, 10^5].
//   Hide Hint #1  
//Traverse the first tree in list1 and the second tree in list2.
//   Hide Hint #2  
//Merge the two trees in one list and sort it.

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
    public List<Integer> getAllElements(TreeNode root1, TreeNode root2) {
        ArrayList<Integer> l1 = new ArrayList<>();
        ArrayList<Integer> l2 = new ArrayList<>();
        dfs(root1, l1);
        dfs(root2, l2);
        l1.addAll(l1.size(), l2);
        Collections.sort(l1);
        return l1;
    }
    
    public void dfs(TreeNode root, ArrayList<Integer> l) {
        if(root != null) {
            l.add(root.val);
        } else {
            return ;
        }
        if(root.left != null) {
            dfs(root.left, l);
        }
        if(root.right != null) {
            dfs(root.right, l);
        }
        return ;
    }
}