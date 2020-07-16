//Given a set of distinct integers, nums, return all possible subsets (the power set).
//
//Note: The solution set must not contain duplicate subsets.
//
//Example:
//
//Input: nums = [1,2,3]
//Output:
//[
//  [3],
//  [1],
//  [2],
//  [1,2,3],
//  [1,3],
//  [2,3],
//  [1,2],
//  []
//]

class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        
        vector<vector<int>> ans;
        ans.push_back({});
        for(int num : nums) {
            vector<vector<int>> temp;
            for(vector<int> n : ans) {
                n.push_back(num);
                temp.push_back(n);
            }
            for(vector<int> t : temp) {
                ans.push_back(t);
            }
        }
        return ans;
        
        /*
        int n = nums.size();
        vector<vector<int>> ans;
        for(int i = 1 << n; i < 1 << (n + 1); i++) {
            vector<int> bm;
            int no = i;
            while(no > 0) {
                bm.push_back(no & 1);
                no >>= 1;
            }
            vector<int> t;
            for(int j = 0; j < n; j++) {
                if(bm[j] == 1) {
                    t.push_back(nums[j]);
                }
            }
            ans.push_back(t);
        }
        return ans;
        */
    }
};