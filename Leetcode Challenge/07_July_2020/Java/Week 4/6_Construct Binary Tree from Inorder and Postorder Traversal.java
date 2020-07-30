//Given inorder and postorder traversal of a tree, construct the binary tree.
//
//Note:
//You may assume that duplicates do not exist in the tree.
//
//For example, given
//
//inorder = [9,3,15,20,7]
//postorder = [9,15,7,20,3]
//Return the following binary tree:
//
//    3
//   / \
//  9  20
//    /  \
//   15   7

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
    public TreeNode buildTree(int[] inorder, int[] postorder) {
        HashMap<Integer, Integer> indexes = new HashMap<>();
        for(int i = 0; i < inorder.length; i++) {
            indexes.put(inorder[i], i);
        }
        return aux(inorder, postorder, 0, inorder.length - 1, postorder.length - 1, indexes);
    }
    public TreeNode aux(int[] inorder, int[] postorder, int start, int end, int index, HashMap<Integer, Integer> indexes) {
        if(start > end) {
            return null;
        }
        TreeNode root = new TreeNode(postorder[index]);
        int rootIndex = indexes.get(root.val);
        root.right = aux(inorder, postorder, rootIndex + 1, end, index - 1, indexes);
        root.left = aux(inorder, postorder, start, rootIndex - 1, index - (end - rootIndex) - 1, indexes);
        return root;
    }
}