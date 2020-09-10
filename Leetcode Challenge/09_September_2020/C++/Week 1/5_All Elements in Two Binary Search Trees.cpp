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
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
        vector<int> l1;
        vector<int> l2;
        dfs(root1, l1);
        dfs(root2, l2);
        l1.insert(l1.end(), l2.begin(), l2.end());
        sort(l1.begin(), l1.end());
        return l1;
    }
    
    void dfs(TreeNode* root, vector<int> &l) {
        if(root != nullptr) {
            l.push_back(root->val);
        } else {
            return ;
        }
        if(root->left != nullptr) {
            dfs(root->left, l);
        }
        if(root->right != nullptr) {
            dfs(root->right, l);
        }
        return ;
    }
};