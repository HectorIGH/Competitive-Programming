//Given a set of distinct integers, nums, return all possible subsets (the power set).
//
//Note: The solution set must not contain duplicate subsets.
//
//Example:
//
//Input: nums = [1,2,3]
//Output:
//[
//  [3],
//  [1],
//  [2],
//  [1,2,3],
//  [1,3],
//  [2,3],
//  [1,2],
//  []
//]

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        let mut ans : Vec<Vec<i32>> = Vec::new();
        ans.push(Vec::new());
        let mut n;
        for num in nums {
            let mut temp : Vec<Vec<i32>> = Vec::new();
            for N in &ans {
                n = N.clone();
                n.push(num);
                temp.push(n);
            }
            ans.append(&mut temp);
        }
        return ans;
        
        /*
        let n = nums.len();
        let mut ans : Vec<Vec<i32>> = Vec::new();
        for i in (1 << n)..(1 << (n + 1)) {
            let mut bm : Vec<i8> = Vec::new();
            let mut no : i32 = i as i32;
            while no > 0 {
                bm.push((no & 1) as i8);
                no >>= 1;
            }
            let mut t : Vec<i32> = Vec::new();
            for j in 0..n {
                if bm[j] == 1 {
                    t.push(nums[j]);
                }
            }
            ans.push(t);
        }
        return ans;
        */
    }
}