//Find all possible combinations of k numbers that add up to a number n, given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.
//
//Note:
//
//All numbers will be positive integers.
//The solution set must not contain duplicate combinations.
//Example 1:
//
//Input: k = 3, n = 7
//Output: [[1,2,4]]
//Example 2:
//
//Input: k = 3, n = 9
//Output: [[1,2,6], [1,3,5], [2,3,4]]

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut comb: Vec<i32> = Vec::new();
        Solution::backtrack(k as usize, &mut results, n, &mut comb, 0);
        return results;
    }
    
    pub fn backtrack(k: usize, results: &mut Vec<Vec<i32>>, remain: i32, comb: &mut Vec<i32>, next_start: i32) {
        if remain == 0 && comb.len() == k {
            results.push(comb.to_vec());
            return ;
        } else if remain < 0 || comb.len() == k {
            return;
        }
        for i in next_start..9 {
            comb.push(i + 1);
            Solution::backtrack(k, results, remain - i - 1, comb, i + 1);
            comb.pop();
        }
    }
}