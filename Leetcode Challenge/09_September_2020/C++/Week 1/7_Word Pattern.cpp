//Given a pattern and a string str, find if str follows the same pattern.
//
//Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in str.
//
//Example 1:
//
//Input: pattern = "abba", str = "dog cat cat dog"
//Output: true
//Example 2:
//
//Input:pattern = "abba", str = "dog cat cat fish"
//Output: false
//Example 3:
//
//Input: pattern = "aaaa", str = "dog cat cat dog"
//Output: false
//Example 4:
//
//Input: pattern = "abba", str = "dog dog dog dog"
//Output: false
//Notes:
//You may assume pattern contains only lowercase letters, and str contains lowercase letters that may be separated by a single space.

class Solution {
public:
    bool wordPattern(string pattern, string strs) {
        istringstream ss(strs);
        string w;
        vector<string> str;
        while(ss >> w) {
            str.push_back(w);
        }
        if(str.size() != pattern.size()) {
            return false;
        }
        unordered_map<string, string> w_map;
        unordered_map<string, string> c_map;
        string c = "";
        w = "";
        for(int i = 0; i < str.size(); i++) {
            c = pattern[i];
            w = str[i];
            if(c_map.count(c)) {
                if(w != c_map[c]) {
                    return false;
                }
            } else {
                if(w_map.count(w)) {
                    return false;
                }
                w_map[w] = c;
                c_map[c] = w;
            }
        }
        return true;
    }
};