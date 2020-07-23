//Given a binary tree, return the zigzag level order traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
//
//For example:
//Given binary tree [3,9,20,null,null,15,7],
//    3
//   / \
//  9  20
//    /  \
//   15   7
//return its zigzag level order traversal as:
//[
//  [3],
//  [20,9],
//  [15,7]
//]

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
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        if(root == nullptr) {
            return {};
        }
        vector<vector<int>> ans;
        deque<TreeNode*> q;
        q.push_back(root);
        bool dir = false;
        while(!q.empty()) {
            vector<int> level;
            int tam = q.size();
            while(tam--) {
                TreeNode* node = q.front();
                q.pop_front();
                level.push_back(node->val);
                if(node->left != nullptr) {
                    q.push_back(node->left);
                }
                if(node->right != nullptr) {
                    q.push_back(node->right);
                }
            }
            if(dir) {
                reverse(level.begin(), level.end());
            }
            ans.push_back(level);
            dir = !dir;
        }
        return ans;
    }
};