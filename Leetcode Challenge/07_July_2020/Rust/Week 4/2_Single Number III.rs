//Given an array of numbers nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once.
//
//Example:
//
//Input:  [1,2,1,3,2,5]
//Output: [3,5]
//Note:
//
//The order of the result is not important. So in the above example, [5, 3] is also correct.
//Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?


impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut s = 0;
        let mut leastbit = 0;
        let mut num1 = 0;
        for e in &nums {
            s ^= e;
        }
        leastbit = s & (s - 1) ^ s;
        for e in & nums {
            if e & leastbit != 0 {
                num1 ^= e;
            }
        }
        return vec![num1, s ^ num1];
    }
}