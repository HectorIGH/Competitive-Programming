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

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        /*
        if prices.len() <= 1 {
            0
        } else {
            let mut n: usize = prices.len();
            let mut k:usize = 2;
            let mut B: Vec<i32> = Vec::with_capacity(n - 1);
            for i in 0..(n - 1) {
                B.push(prices[i + 1] - prices[i]);
            }
            if(k > n / 2) {
                //let mut sum: i32 = 0;
                //for x in &B {
                    //if *x > 0 {
                        //sum += *x;
                    //}
                //}
                let sum: i32 = B.iter().map(|&x: &i32|
                    if x < 0 {0}
                    else {x}
                ).sum();
                sum
            }  else {
                let mut dp: Vec<Vec<i32>> = vec![vec![0; k + 1]; n - 1];
                let mut mp: Vec<Vec<i32>> = vec![vec![0; k + 1]; n - 1];
                dp[0][1] = B[0];
                mp[0][1] = B[0];
                for i in 1..(n - 1) {
                    for j in 1..(k + 1) {
                        dp[i][j] = max(mp[i-1][j-1], dp[i-1][j]) + B[i];
                        mp[i][j] = max(dp[i][j], mp[i-1][j]);
                    }
                }
                *mp[n - 2].iter().max().expect("")
            }
        }
        */
        let mut firstbuy:i32 = std::i32::MIN;
        let mut firstsell:i32 = 0;
        let mut secondbuy:i32 = std::i32::MIN;
        let mut secondsell:i32 = 0;
        for price in prices {
            firstbuy = max(firstbuy , -price);
            firstsell = max( firstsell , price + firstbuy);
            secondbuy = max(secondbuy , firstsell - price);
            secondsell = max(secondsell , secondbuy + price);
        }
        secondsell
    }
}