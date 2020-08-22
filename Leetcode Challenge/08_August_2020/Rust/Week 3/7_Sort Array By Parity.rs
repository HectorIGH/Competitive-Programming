//Given an array A of non-negative integers, return an array consisting of all the even elements of A, followed by all the odd elements of A.
//
//You may return any answer array that satisfies this condition.
//
// 
//
//Example 1:
//
//Input: [3,1,2,4]
//Output: [2,4,3,1]
//The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
// 
//
//Note:
//
//1 <= A.length <= 5000
//0 <= A[i] <= 5000

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        /*
        let mut ans: Vec<i32> = Vec::new();
        for n in a {
            if (n & 1) != 0 {
                ans.push(n);
            } else {
                ans.insert(0, n);
            }
        }
        ans
        */
        let mut l:usize = a.len() - 1;
        let mut i:usize = 0;
        while i < l {
            if a[i] & 1 != 0 {
                a.swap(i, l);
                i -= 1;
                l -= 1;
            }// else {}
            i += 1;
        }
        a
    }
}