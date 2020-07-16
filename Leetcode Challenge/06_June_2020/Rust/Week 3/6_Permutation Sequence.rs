//The set [1,2,3,...,n] contains a total of n! unique permutations.
//
//By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
//
//"123"
//"132"
//"213"
//"231"
//"312"
//"321"
//Given n and k, return the kth permutation sequence.
//
//Note:
//
//Given n will be between 1 and 9 inclusive.
//Given k will be between 1 and n! inclusive.
//Example 1:
//
//Input: n = 3, k = 3
//Output: "213"
//Example 2:
//
//Input: n = 4, k = 9
//Output: "2314"

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut factorial = vec![1; n as usize];
        let mut l : Vec<i32> = vec![];
        l.push(1);
        let mut k = k - 1;
        let mut j = 0;
        let mut ans : String = "".to_owned();
        for i in 1..n {
            factorial[i as usize] = i * factorial[i as usize - 1];
            l.push(i + 1);
        }
        for i in (0..n as usize).rev() {
            j = k / factorial[i];
            k -= j * factorial[i];
            
            ans.push_str(&l[j as usize].to_string());
            l.remove(j as usize);
        }
        return ans;
    }
}