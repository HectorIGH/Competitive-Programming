//Given an array of 4 digits, return the largest 24 hour time that can be made.
//
//The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
//
//Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
//
// 
//
//Example 1:
//
//Input: [1,2,3,4]
//Output: "23:41"
//Example 2:
//
//Input: [5,5,5,5]
//Output: ""
// 
//
//Note:
//
//A.length == 4
//0 <= A[i] <= 9

class Solution {
public:
    string largestTimeFromDigits(vector<int>& A) {
        sort(A.begin(), A.end());
        vector<vector<int>> perm;
        vector<int> indexes = {0, 1, 2, 3};
        perms(indexes, 4, perm);
        int out = -1;
        string ans = "";
        int w, x, y, z;
        for(vector<int> i : perm) {
            w = i[0];
            x = i[1];
            y = i[2];
            z = i[3];
            if(A[w] * 10 + A[x] <= 23 && A[y] <= 5) {
                out = max(out, 60 * (A[w] * 10 + A[x]) + 10 * A[y] + A[z]);
            }
        }
        if(out == -1) {
            return "";
        }
        int h = (int)(out / 60);
        int m = out % 60;
        if(h < 10) {
            ans += '0';
        }
        ans += to_string(h) + ':';
        if(m < 10) {
            ans += '0';
        }
        ans += to_string(m);
        return ans;
    }
    
    void perms(vector<int> a, int n, vector<vector<int>>& p){
        if(n == 1) {
            p.push_back(a);
            return ;
        }
        for(int i = 0; i < n; i++) {
            swap(a[i], a[n - 1]);
            perms(a, n - 1, p);
            swap(a[i], a[n - 1]);
        }
    }
};