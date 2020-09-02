//Given an array of 4 digits, return the largest 24 hour time that can be made.
//
//The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.
//
//Return the answer as a string of length 5.  If no valid time can be made, return an empty string.
//
// 
//
//Example 1:
//
//Input: [1,2,3,4]
//Output: "23:41"
//Example 2:
//
//Input: [5,5,5,5]
//Output: ""
// 
//
//Note:
//
//A.length == 4
//0 <= A[i] <= 9

use std::cmp::max;

impl Solution {
    pub fn largest_time_from_digits(mut a: Vec<i32>) -> String {
        a.sort_unstable();
        let mut perm: Vec<Vec<usize>> = Vec::new();
        let mut indexes: Vec<usize> = vec![0, 1, 2, 3];
        Solution::perms(&mut indexes, 4, &mut perm);
        let mut out: i32 = -1;
        let mut ans: String = String::new();
        let mut w: usize = 0;
        let mut x: usize = 0;
        let mut y: usize = 0;
        let mut z: usize = 0;
        for i in perm {
            w = i[0];
            x = i[1];
            y = i[2];
            z = i[3];
            if a[w] * 10 + a[x] <= 23 && a[y] <= 5 {
                out = max(out, 60 * (a[w] * 10 + a[x]) + 10 * a[y] + a[z]);
            }
        }
        if out == -1 {
            return ans;
        }
        let mut h: i32  = out / 60;
        let mut m: i32 = out % 60;
        if h < 10 {
            ans.push_str("0");
        }
        ans.push_str(&h.to_string());
        ans.push_str(":");
        if m < 10 {
            ans.push_str("0");
        }
        ans.push_str(&m.to_string());
        ans
    }
    
    pub fn perms(mut a: &mut Vec<usize>, n: usize, mut p: &mut Vec<Vec<usize>>){
        if n == 1 {
            p.push(a.clone());
            return ;
        }
        for i in 0..n {
            a.swap(i, n - 1);
            Solution::perms(a, n - 1, p);
            a.swap(i, n - 1);
        }
    }
}