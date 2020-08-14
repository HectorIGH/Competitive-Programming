//Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
//
//Note that the row index starts from 0.
//
//
//In Pascal's triangle, each number is the sum of the two numbers directly above it.
//
//Example:
//
//Input: 3
//Output: [1,3,3,1]
//Follow up:
//
//Could you optimize your algorithm to use only O(k) extra space?

class Solution {
public:
    vector<int> getRow(int rowIndex) {
        int n = rowIndex;
        vector<int> ans(n + 1, 1);
        vector<long long> tes(n / 2 + 1, 1);
        for(int i = 1; i < n / 2 + 1; i++) {
            tes[i] = tes[i - 1] * (n - i + 1) / i;
            ans[i] = tes[i];
        }
        //copy(tes.begin(), tes.end(), ostream_iterator<int>(cout,","));
        
        for(int i = (int)n / 2 + 1; i <= n; i++) {
            ans[i] = ans[n % i];
        }
        return ans;
    }
};