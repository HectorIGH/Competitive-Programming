//Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
//
//Note: The algorithm should run in linear time and in O(1) space.
//
//Example 1:
//
//Input: [3,2,3]
//Output: [3]
//Example 2:
//
//Input: [1,1,1,3,3,2,2,2]
//Output: [1,2]
//   Hide Hint #1  
//How many majority elements could it possibly have?
//Do you have a better hint? Suggest it!

class Solution {
public:
    vector<int> majorityElement(vector<int>& nums) {
        int l = nums.size();
        vector<int> result;
        if(l == 0) {
            return result;
        }
        int count1 = 0;
        int count2 = 0;
        int candidate1;
        int candidate2;
        for(int n : nums) {
            if(candidate1 == n) {
                count1 += 1;
            } else if(candidate2 == n) {
                count2 += 1;
            } else if(count1 == 0) {
                candidate1 = n;
                count1 += 1;
            } else if(count2 == 0) {
                candidate2 = n;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        
        if(count(nums.begin(), nums.end(), candidate1) > l / 3) {
                result.push_back(candidate1);
        }
        if(count(nums.begin(), nums.end(), candidate2) > l / 3) {
                result.push_back(candidate2);
        }
                
        return result;
    }
};