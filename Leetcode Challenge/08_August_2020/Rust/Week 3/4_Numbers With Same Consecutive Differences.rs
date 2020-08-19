//Return all non-negative integers of length N such that the absolute difference between every two consecutive digits is K.
//
//Note that every number in the answer must not have leading zeros except for the number 0 itself. For example, 01 has one leading zero and is invalid, but 0 is valid.
//
//You may return the answer in any order.
//
// 
//
//Example 1:
//
//Input: N = 3, K = 7
//Output: [181,292,707,818,929]
//Explanation: Note that 070 is not a valid number, because it has leading zeroes.
//Example 2:
//
//Input: N = 2, K = 1
//Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
// 
//
//Note:
//
//1 <= N <= 9
//0 <= K <= 9

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            let mut fast: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
            fast
        } else{
            let mut comb: Vec<i32> = Vec::new();
            if k != 0 {
                for i in 0..2_i32.pow(n as u32 - 1) {
                    comb.push(i);
                }
            } else {
                comb.push(0);
            }
            let mut ans: Vec<i32> = Vec::new();
            let mut good: bool = true;
            for i in 1..10 {
                let mut N: Vec<i32> = vec![0; n as usize];
                N[0] = i as i32;
                for d in &comb {
                    let mut c = *d;
                    good = true;
                    let mut m: i32 = i * 10_i32.pow(n as u32 - 1);
                    for t in 1..n {
                        if (c & 1) == 0 {
                            if N[t as usize - 1] - k < 0 {
                                good = false;
                                break;
                            } else {
                                N[t as usize] = N[t as usize - 1] - k;
                                m += (N[t as usize - 1] - k) * 10_i32.pow((n - 1 - t) as u32);
                            }
                        } else {
                            if N[t as usize - 1] + k > 9 {
                                good = false;
                                break;
                            } else {
                                N[t as usize] = N[t as usize - 1] + k;
                                m += (N[t as usize - 1] + k) * 10_i32.pow((n - 1 - t) as u32);
                            }
                        }
                        c >>= 1;
                    }
                    if(good) {
                        ans.push(m);
                    } else {
                        continue;
                    }
                }
            }
            ans
        }
    }
}