//You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.
//
//Given a list of non-negative integers nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
//
// 
//
//Example 1:
//
//Input: nums = [2,3,2]
//Output: 3
//Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
//Example 2:
//
//Input: nums = [1,2,3,1]
//Output: 4
//Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//Total amount you can rob = 1 + 3 = 4.
//Example 3:
//
//Input: nums = [0]
//Output: 0
// 
//
//Constraints:
//
//1 <= nums.length <= 100
//0 <= nums[i] <= 1000
//   Hide Hint #1  
//Since House[1] and House[n] are adjacent, they cannot be robbed together. Therefore, the problem becomes to rob either House[1]-House[n-1] or House[2]-House[n], depending on which choice offers more money. Now the problem has degenerated to the House Robber, which is already been solved.

class Solution {
public:
    int rob(vector<int>& nums) {
        return max(nums[0] + rob_help(nums, 2, nums.size() - 1), rob_help(nums, 1, nums.size()));
    }
    
    int rob_help(vector<int>& nums, int start, int end) {
        int dp1 = 0;
        int dp2 = 0;
        int num = 0;
        int temp = 0;
        for(int i = start; i < end; i++) {
            temp = dp2;
            dp2 = max(dp1 + nums[i], dp2);
            dp1 = temp;
        }
        return dp2;
    }
};