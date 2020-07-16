//Given two binary trees, write a function to check if they are the same or not.
//
//Two binary trees are considered the same if they are structurally identical and the nodes have the same value.
//
//Example 1:
//
//Input:     1         1
//          / \       / \
//         2   3     2   3
//
//        [1,2,3],   [1,2,3]
//
//Output: true
//Example 2:
//
//Input:     1         1
//          /           \
//         2             2
//
//        [1,2],     [1,null,2]
//
//Output: false
//Example 3:
//
//Input:     1         1
//          / \       / \
//         2   1     1   2
//
//        [1,2,1],   [1,1,2]
//
//Output: false

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
    bool isSameTree(TreeNode* p, TreeNode* q) {
        /*
        if(p == nullptr && q == nullptr) {
            return true;
        }
        if(p == nullptr || q == nullptr || p->val != q->val) {
            return false;
        }
        return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
        */
        
        deque<TreeNode*> dq;
        dq.push_back(p);
        dq.push_back(q);
        while(!dq.empty()) {
            p = dq.front();
            dq.pop_front();
            q = dq.front();
            dq.pop_front();
            if(p == nullptr && q == nullptr) {
                continue;
            }
            if(p == nullptr || q == nullptr || p->val != q->val) {
                return false;
            }
            dq.push_back(p->left);
            dq.push_back(q->left);
            dq.push_back(p->right);
            dq.push_back(q->right);
        }
        return true;
        
    }
};