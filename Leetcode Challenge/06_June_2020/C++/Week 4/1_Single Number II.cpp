//Given a non-empty array of integers, every element appears three times except for one, which appears exactly once. Find that single one.
//
//Note:
//
//Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?
//
//Example 1:
//
//Input: [2,2,3,2]
//Output: 3
//Example 2:
//
//Input: [0,1,0,1,0,1,99]
//Output: 99

class Solution {
public:
    int singleNumber(vector<int>& nums) {
        // Bitwise O(1) space
        /*
        int ones = 0, twos = 0, not_threes = 0;
        for(int n : nums) {
            twos |= ones & n;
            ones ^= n;
            not_threes = ~(ones & twos);
            ones &= not_threes;
            twos &= not_threes;
        }
        return ones;
        */
        unordered_map<int, int> freq;
        for(int n : nums) {
            if(freq.find(n) == freq.end()) { // Not found
                freq.insert({n, 1});
            } else { // Found
                freq[n] += 1;
            }
        }
        for(int n : nums) {
            if(freq[n] == 1) {
                return n;
            }
        }
        return 0;
    }
};