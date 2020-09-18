//Say you have an array for which the ith element is the price of a given stock on day i.
//
//If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
//
//Note that you cannot sell a stock before you buy one.
//
//Example 1:
//
//Input: [7,1,5,3,6,4]
//Output: 5
//Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//             Not 7-1 = 6, as selling price needs to be larger than buying price.
//Example 2:
//
//Input: [7,6,4,3,1]
//Output: 0
//Explanation: In this case, no transaction is done, i.e. max profit = 0.

use std::cmp::{max};

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 {
            return 0;
        }
        if prices.len() == 2 {
            return max(0, prices[1] - prices[0]);
        }
        let mut mini: i32 = std::i32::MAX; //i32::MAX;
        let mut maxi: i32 = 0;
        for i in 0..prices.len() {
            if prices[i] < mini {
                mini = prices[i];
            } else if prices[i] - mini > maxi {
                maxi = prices[i] - mini;
            }
        }
        maxi
    }
}