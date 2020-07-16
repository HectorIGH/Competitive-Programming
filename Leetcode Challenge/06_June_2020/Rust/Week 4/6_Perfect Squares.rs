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

use std::cmp;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        /*
        if n < 4 {
            return n;
        }
        let n : usize = n as usize;
        let mut dp = vec![0; n + 1];
        let mut j : usize = 0;
        dp[1] = 1;
        dp[2] = 2;
        dp[3] = 3;
        for i in 4..(n + 1) {
            dp[i] = i as i32;
            j = 0;
            while j * j <= i {
                dp[i] = cmp::min(dp[i], dp[i - j * j] + 1);
                j += 1;
            }
        }
        return dp[n];
        */
        let mut n = n;
        let mut c = 0;
        if n == i32::pow((n as f32).sqrt() as i32, 2) {
            return 1;
        }
        for i in 0..((n as f32).sqrt() as i32 + 1) {
            c = n - i * i;
            if c == i32::pow((c as f32).sqrt() as i32, 2) {
                return 2;
            }
        }
        while n % 4 == 0 {
            n >>= 2;
        }
        if n % 8 == 7 {
            return 4;
        }
        return 3;
    }
}