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
public:
    int maxProfit(vector<int>& prices) {
        if(prices.size() <= 1) {
            return 0;
        }
        int n = prices.size(), k = 2;
        vector<int> B(n - 1, 0);
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
        vector<vector<int>> dp(n - 1, vector<int>(k + 1, 0));
        vector<vector<int>> mp(n - 1, vector<int>(k + 1, 0));
        dp[0][1] = B[0];
        mp[0][1] = B[0];
        for(int i = 1; i < n - 1; i++) {
            for(int j = 1; j < k + 1; j++) {
                dp[i][j] = max(mp[i-1][j-1], dp[i-1][j]) + B[i];
                mp[i][j] = max(dp[i][j], mp[i-1][j]);
            }
        }
        return *max_element(mp[n - 2].begin(), mp[n - 2].end());
        /*
        int firstbuy = INT_MIN;
        int firstsell = 0;
        int secondbuy = INT_MIN;
        int secondsell = 0;
        for(int price : prices) {
            firstbuy = max(firstbuy , -price);
            firstsell = max( firstsell , price + firstbuy);
            secondbuy = max(secondbuy , firstsell - price);
            secondsell = max(secondsell , secondbuy + price);
        }
        return secondsell;
        */
    }
};