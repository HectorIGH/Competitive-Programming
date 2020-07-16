//Given a positive integer n, find the least number of perfect square numbers (for example, 1, 4, 9, 16, ...) which sum to n.
//
//Example 1:
//
//Input: n = 12
//Output: 3 
//Explanation: 12 = 4 + 4 + 4.
//Example 2:
//
//Input: n = 13
//Output: 2
//Explanation: 13 = 4 + 9.

class Solution {
    public int numSquares(int n) {
        /*
        if(n < 4) {
            return n;
        }
        int[] dp = new int[n + 1];
        int j = 0;
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for(int i = 4; i < n + 1; i++) {
            dp[i] = i;
            j = 0;
            while(j * j <= i) {
                dp[i] = Math.min(dp[i], dp[i - j * j] + 1);
                j++;
            }
        }
        return dp[n];
        */
        int c = 0;
        if(n == Math.pow((int)Math.sqrt(n), 2)) {
            return 1;
        }
        for(int i = 0; i < Math.sqrt(n); i++) {
            c = n - i * i;
            if(c == Math.pow((int)Math.sqrt(c), 2)) {
                return 2;
            }
        }
        while(n % 4 == 0) {
            n >>= 2;
        }
        if(n % 8 == 7) {
            return 4;
        }
        return 3;
    }
}