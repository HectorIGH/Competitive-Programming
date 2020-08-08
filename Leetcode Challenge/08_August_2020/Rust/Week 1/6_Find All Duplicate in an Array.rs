//Given an array of integers, 1 ≤ a[i] ≤ n (n = size of array), some elements appear twice and others appear once.
//
//Find all the elements that appear twice in this array.
//
//Could you do it without extra space and in O(n) runtime?
//
//Example:
//Input:
//[4,3,2,7,8,2,3,1]
//
//Output:
//[2,3]

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            let n = nums[i];
            if nums[n.abs() as usize - 1] < 0 {
                ans.push(n.abs());
            } else {
                nums[n.abs() as usize - 1] *= -1;
            }
        }
        ans
    }
}