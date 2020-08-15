//Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.
//
//This is case sensitive, for example "Aa" is not considered a palindrome here.
//
//Note:
//Assume the length of given string will not exceed 1,010.
//
//Example:
//
//Input:
//"abccccdd"
//
//Output:
//7
//
//Explanation:
//One longest palindrome that can be built is "dccaccd", whose length is 7.

class Solution {
public:
    int longestPalindrome(string s) {
        //unordered_map<char, int> f;
        vector<int> g(58, 0);
        bool odd = false;
        int ans = 0;
        for(char l : s) {
            //f[l]++;
            g[(int)l - 65]++;
        }
            
        for(int l : g) {
        //for(auto& x : f) {
            //int l = x.second;
            if((l & 1) != 0) {
                if(!odd) {
                    ans += l;
                    odd = true;
                    continue;
                } else {
                    odd = true;
                }
            }
            ans += ((l >> 1) << 1);
        }
        return ans;
    }
};