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

use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        /*
        if n <= 2 {
            n
        } else {
            let n : usize = n as usize;
            let mut b : Vec<i32> = vec![0; n + 1];
            b[1] = 1;
            b[2] = 2;
            for i in 3..(n + 1) {
                b[i] = b[i - 1] + b[i - 2];
            }
            b[n]
        }
        */
        let mut memo : HashMap<i32, i32> = HashMap::new();
        memo.insert(1, 1);
        memo.insert(2, 2);
        Solution::aux(n, &mut memo)
    }
    pub fn aux(n : i32, memo : &mut HashMap<i32, i32>) -> i32 {
        if memo.contains_key(&n) {
            memo[&n]
        } else {
            let m1 : i32 = Solution::aux(n - 1, memo);
            let m2 : i32 = Solution::aux(n - 2, memo);
            memo.insert(n - 1, m1);
            memo.insert(n - 2, m2);
            m1 + m2
        }
    }
}