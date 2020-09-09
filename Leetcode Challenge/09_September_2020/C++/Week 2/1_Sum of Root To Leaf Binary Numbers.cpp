//Given a binary tree, each node has value 0 or 1.  Each root-to-leaf path represents a binary number starting with the most significant bit.  For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.
//
//For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.
//
//Return the sum of these numbers.
//
// 
//
//Example 1:
//
//
//
//Input: [1,0,1,0,1,0,1]
//Output: 22
//Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
// 
//
//Note:
//
//The number of nodes in the tree is between 1 and 1000.
//node.val is 0 or 1.
//The answer will not exceed 2^31 - 1.
//   Hide Hint #1  
//Find each path, then transform that path to an integer in base 10.


/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int l = 0;
    int sumRootToLeaf(TreeNode* root) {
        dfs(root, 0);
        return l;
    }
    
    void dfs(TreeNode* root, int s) {
        if(root == nullptr) {
            return;
        }
        if(root->left == nullptr && root->right == nullptr) {
            l += s << 1 | root->val;
        }
        if(root->left != nullptr) {
            dfs(root->left, s << 1 | root->val);
        }
        if(root->right != nullptr) {
            dfs(root->right, s << 1 | root->val);
        }
    }
};