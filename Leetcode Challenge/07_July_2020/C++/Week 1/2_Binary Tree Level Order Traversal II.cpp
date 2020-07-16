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
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        
        // BFS
        if(root == nullptr) {
            return {};
        }
        vector<vector<int>> ans;
        queue<TreeNode*> dq;
        dq.push(root);
        
        while(!dq.empty()) {
            int n = dq.size();
            vector<int> holder;
            for(int i = 0; i < n; i++) {
                auto node = dq.front();
                holder.push_back(node->val);
                dq.pop();
                if(node->left) {
                    dq.push(node->left);
                }
                if(node->right) {
                    dq.push(node->right);
                }
            }
            ans.push_back(holder);
        }
        reverse(ans.begin(), ans.end());
        return ans;
        /*
        // DFS
        if(!root) {
            return {};
        }
        vector<vector<int>> ans;
        dfs(root, 0, ans);
        reverse(ans.begin(), ans.end());
        return ans;
        */
    }
    /*
    void dfs(TreeNode* root, int level, vector<vector<int>>& ans) {
        if(!root) {
            return;
        }
        if(level == ans.size()) {
            ans.push_back({});
        }
        ans[level].push_back(root->val);
        dfs(root->left, level + 1, ans);
        dfs(root->right, level + 1, ans);
    }
    */
};