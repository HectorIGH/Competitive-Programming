//Say you have an array for which the ith element is the price of a given stock on day i.
//
//Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times) with the following restrictions:
//
//You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
//After you sell your stock, you cannot buy stock on next day. (ie, cooldown 1 day)
//Example:
//
//Input: [1,2,3,0,2]
//Output: 3 
//Explanation: transactions = [buy, sell, cooldown, buy, sell]

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        if prices.len() == 2 {
            return max(0, prices[1] - prices[0]);
        }
        let n = prices.len();
        let mut dp : Vec<Vec<i32>> = vec![vec![0; 2]; n];
        dp[0][0] = 0;
        dp[0][1] = -prices[0];
        dp[1][0] = max(dp[0][1] + prices[1], dp[0][0]);
        dp[1][1] = max(dp[0][0] - prices[1], dp[0][1]);
        for i in 2..n {
            dp[i][0] = max(dp[i - 1][1] + prices[i], dp[i - 1][0]);
            dp[i][1] = max(dp[i - 2][0] - prices[i], dp[i - 1][1]);
        }
        dp[n - 1][0]
    }
}