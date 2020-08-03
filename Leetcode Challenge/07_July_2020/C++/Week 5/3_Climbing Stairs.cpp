//You are climbing a stair case. It takes n steps to reach to the top.
//
//Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
//Example 1:
//
//Input: 2
//Output: 2
//Explanation: There are two ways to climb to the top.
//1. 1 step + 1 step
//2. 2 steps
//Example 2:
//
//Input: 3
//Output: 3
//Explanation: There are three ways to climb to the top.
//1. 1 step + 1 step + 1 step
//2. 1 step + 2 steps
//3. 2 steps + 1 step
// 
//
//Constraints:
//
//1 <= n <= 45
//   Hide Hint #1  
//To reach nth step, what could have been your previous steps? (Think about the step sizes)

class Solution {
public:
    int climbStairs(int n) {
        /*
        if(n <= 2) {
            return n;
        }
        vector<int> b(n + 1);
        b[1] = 1;
        b[2] = 2;
        for(int i = 3; i < n + 1; i++) {
            b[i] = b[i - 1] + b[i - 2];
        }
        return b[n];
        /*/
        unordered_map<int, int> memo;
        memo[1] = 1;
        memo[2] = 2;
        return aux(n, memo);
        
    }
    
    int aux(int n, unordered_map<int, int>& memo) {
        if(memo.count(n)) {
            return memo[n];
        } else {
            memo[n - 1] = aux(n - 1, memo);
            memo[n - 2] = aux(n - 2, memo);
            return memo[n - 1] + memo[n - 2];
        }
    }
};