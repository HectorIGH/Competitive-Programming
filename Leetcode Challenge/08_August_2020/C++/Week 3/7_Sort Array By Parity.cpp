//Given an array A of non-negative integers, return an array consisting of all the even elements of A, followed by all the odd elements of A.
//
//You may return any answer array that satisfies this condition.
//
// 
//
//Example 1:
//
//Input: [3,1,2,4]
//Output: [2,4,3,1]
//The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
// 
//
//Note:
//
//1 <= A.length <= 5000
//0 <= A[i] <= 5000

class Solution {
public:
    vector<int> sortArrayByParity(vector<int>& A) {
        /*vector<int> ans;
        for(int n : A) {
            if((n & 1) != 0) {
                ans.push_back(n);
            } else {
                ans.emplace(ans.begin(), n);
            }
        }
        return ans;
        */
        int l = A.size() - 1;
        for(int i = 0; i < l; i++) {
            if((A[i] & 1) != 0) {
                swap(A[i--], A[l--]);
                //i--;
                //l--;
            }
        }
        return A;
    }
};