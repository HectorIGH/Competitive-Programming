//Say you have an array for which the ith element is the price of a given stock on day i.
//
//Design an algorithm to find the maximum profit. You may complete at most two transactions.
//
//Note: You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).
//
//Example 1:
//
//Input: [3,3,5,0,0,3,1,4]
//Output: 6
//Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
//             Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
//Example 2:
//
//Input: [1,2,3,4,5]
//Output: 4
//Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
//             Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
//             engaging multiple transactions at the same time. You must sell before buying again.
//Example 3:
//
//Input: [7,6,4,3,1]
//Output: 0
//Explanation: In this case, no transaction is done, i.e. max profit = 0.

class Solution {
    public int maxProfit(int[] prices) {
        /*
        if(prices.length <= 1) {
            return 0;
        }
        int n = prices.length, k = 2;
        int[] B = new int[n - 1];
        for(int i = 0; i < n - 1; i++) {
            B[i] = prices[i + 1] - prices[i];
        }
        if(k > n / 2) {
            int sum = 0;
            for(int x : B) {
                if(x > 0) {
                    sum += x;
                }
            }
            return sum;
        }
        Integer[][] dp = new Integer[n - 1][k + 1];
        Integer[][] mp = new Integer[n - 1][k + 1];
        for(Integer[] row : dp) {
            Arrays.fill(row, 0);
        }
        for(Integer[] row : mp) {
            Arrays.fill(row, 0);
        }
        dp[0][1] = B[0];
        mp[0][1] = B[0];
        for(int i = 1; i < n - 1; i++) {
            for(int j = 1; j < k + 1; j++) {
                dp[i][j] = Math.max(mp[i-1][j-1], dp[i-1][j]) + B[i];
                mp[i][j] = Math.max(dp[i][j], mp[i-1][j]);
            }
        }
        return Collections.max(Arrays.asList(mp[n - 2]));
        */
        int firstbuy = Integer.MIN_VALUE;
        int firstsell = 0;
        int secondbuy = Integer.MIN_VALUE;
        int secondsell = 0;
        for(int price : prices) {
            firstbuy = Math.max(firstbuy , -price);
            firstsell = Math.max( firstsell , price + firstbuy);
            secondbuy = Math.max(secondbuy , firstsell - price);
            secondsell = Math.max(secondsell , secondbuy + price);
        }
        return secondsell;
    }
}