//Find all possible combinations of k numbers that add up to a number n, given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.
//
//Note:
//
//All numbers will be positive integers.
//The solution set must not contain duplicate combinations.
//Example 1:
//
//Input: k = 3, n = 7
//Output: [[1,2,4]]
//Example 2:
//
//Input: k = 3, n = 9
//Output: [[1,2,6], [1,3,5], [2,3,4]]

class Solution {
public:
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<vector<int>> results;
        backtrack(k, results, n, {}, 0);
        return results;
    }
    
    void backtrack(int k, vector<vector<int>> &results, int remain, vector<int> comb, int next_start) {
        if(remain == 0 && comb.size() == k) {
            results.push_back(comb);
            return ;
        } else if(remain < 0 || comb.size() == k) {
            return;
        }
        for (int i = next_start; i < 9; i++) {
            comb.push_back(i + 1);
            backtrack(k, results, remain - i - 1, comb, i + 1);
            comb.pop_back();
        }
    }
};