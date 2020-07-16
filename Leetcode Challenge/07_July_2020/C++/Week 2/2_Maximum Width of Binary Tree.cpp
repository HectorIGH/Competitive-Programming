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
    int widthOfBinaryTree(TreeNode* root) {
        /*
        // BFS
        int width = 0, n = 0;
        deque<pair<TreeNode*, unsigned long long>> q;
        q.push_back({root, 1});
        while(!q.empty()) {
            n = q.size();
            width = max(width, (int)(q.back().second - q.front().second + 1));
            while(n) {
                auto node = q.front();
                q.pop_front();
                if(node.first->left) {
                    q.push_back({node.first->left, 2 * node.second});
                }
                if(node.first->right) {
                    q.push_back({node.first->right, 2 * node.second + 1});
                }
                n--;
            }
        }
        return width;
        */
        
        // DFS
        vector<unsigned long> levelLMN;
        return dfs(root, 1, 0, levelLMN);
        
    }
    
    unsigned long dfs(TreeNode* root, unsigned long id, int level, vector<unsigned long>& levelLMN) {
        if(root == nullptr) {
            return 0;
        }
        if(level == levelLMN.size()) {
            levelLMN.push_back(id);
        }
        return max(id + 1 - levelLMN[level],
                   max(dfs(root->left, 2 * id, level + 1, levelLMN),
                       dfs(root->right, 2 * id + 1, level + 1, levelLMN)));
    }
    
};