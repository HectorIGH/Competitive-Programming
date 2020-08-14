//Given a non-negative index k where k ≤ 33, return the kth index row of the Pascal's triangle.
//
//Note that the row index starts from 0.
//
//
//In Pascal's triangle, each number is the sum of the two numbers directly above it.
//
//Example:
//
//Input: 3
//Output: [1,3,3,1]
//Follow up:
//
//Could you optimize your algorithm to use only O(k) extra space?

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut ans: Vec<i32> = vec![1; n + 1];
        let mut tes: Vec<f64> = vec![1.0; n / 2 + 1];
        
        for i in 1..(n / 2 + 1) {
            let j: f64 = i as f64;
            let m: f64 = n as f64;
            tes[i] = tes[i - 1] * (m - j + 1.0) / j;
            ans[i] = tes[i] as i32;
        }
        
        for i in (n / 2 + 1)..= n {
            ans[i] = ans[n % i];
        }
            
        return ans;
    }
}