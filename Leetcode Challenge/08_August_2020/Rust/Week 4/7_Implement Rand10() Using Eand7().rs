//Given a function rand7 which generates a uniform random integer in the range 1 to 7, write a function rand10 which generates a uniform random integer in the range 1 to 10.
//
//Do NOT use system's Math.random().
//
// 
//
//Example 1:
//
//Input: 1
//Output: [7]
//Example 2:
//
//Input: 2
//Output: [8,4]
//Example 3:
//
//Input: 3
//Output: [8,1,10]
// 
//
//Note:
//
//rand7 is predefined.
//Each testcase has one argument: n, the number of times that rand10 is called.
// 
//
//Follow up:
//
//What is the expected value for the number of calls to rand7() function?
//Could you minimize the number of calls to rand7()?

use rand::{thread_rng};

fn rand_7() -> i32 {
    // Leetcode implementation returned 0..6
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 8)
}

/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut n: i32 = 0;
        loop {
            a = rand_7();
            b = rand_7();
            n = a + (b - 1) * 7;
            if(n <= 40) {
                return 1 + (n - 1) % 10;
            }
            a = rand_7();
            b = n - 40;
            n = a + (b - 1) * 7;
            if(n <= 60) {
                return 1 + (n - 1) % 10;
            }
            a = rand_7();
            b = n - 60;
            n = a + (b - 1) * 7;
            if(n <= 20) {
                return 1 + (n - 1) % 10;
            }
        }
    }
}