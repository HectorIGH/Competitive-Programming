//Given a list of non negative integers, arrange them such that they form the largest number.
//
//Example 1:
//
//Input: [10,2]
//Output: "210"
//Example 2:
//
//Input: [3,30,34,5,9]
//Output: "9534330"
//Note: The result may be very large, so you need to return a string instead of an integer.

class Solution {
public:
    
    static bool cmp(string &x, string &y) {
        return x + y > y + x;
    }
    
    string largestNumber(vector<int>& nums) {
        vector<string> s(nums.size(), "");
        string ans = "";
        for(int i = 0; i < nums.size(); i++) {
            s[i] = to_string(nums[i]);
        }
        sort(s.begin(), s.end(), cmp);
        if(s[0] == "0") {
            return "0";
        } else {
            for(string e : s) {
                ans += e;
            }
            return ans;
        }
    }
};