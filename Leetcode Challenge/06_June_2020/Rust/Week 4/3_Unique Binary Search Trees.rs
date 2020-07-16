//Given n, how many structurally unique BST's (binary search trees) that store values 1 ... n?
//
//Example:
//
//Input: 3
//Output: 5
//Explanation:
//Given n = 3, there are a total of 5 unique BST's:
//
//   1         3     3      2      1
//    \       /     /      / \      \
//     3     2     1      1   3      2
//    /     /       \                 \
//   2     1         2                 3
// 
//
//Constraints:
//
//1 <= n <= 19

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n : usize = n as usize;
        let mut dp : Vec<i32> = vec![0; (n + 1)];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..(n + 1) {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }
        dp[n]
    }
}