//Your are given an array of positive integers nums.
//
//Count and print the number of (contiguous) subarrays where the product of all the elements in the subarray is less than k.
//
//Example 1:
//Input: nums = [10, 5, 2, 6], k = 100
//Output: 8
//Explanation: The 8 subarrays that have product less than 100 are: [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6].
//Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
//Note:
//
//0 < nums.length <= 50000.
//0 < nums[i] < 1000.
//0 <= k < 10^6.
//   Hide Hint #1  
//For each j, let opt(j) be the smallest i so that nums[i] * nums[i+1] * ... * nums[j] is less than k. opt is an increasing function.

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        
        if k <= 1 {
            return 0;
        }
        let mut prod: i32 = 1;
        let mut ans: usize = 0;
        let mut left:usize = 0;
        for (right, val) in nums.iter().enumerate() {
            prod *= val;
            while(prod >= k && left <= right) {
                prod /= nums[left];
                left += 1;
            }
            ans += right - left + 1;
        }
        return ans as i32;
        
        /*
        if k <= 1 {
            return 0;
        }
        let lk: f64 = (k as f64).log10();
        let mut prefix: Vec<f64> = vec![0.0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix[i + 1] = prefix[i] + (nums[i] as f64).log10();
        }
        let mut ans: usize = 0;
        let mut j: usize = 0;
        //for (i, x) in prefix.iter().enumerate() {
        //    j = prefix.binary_search_by(|a| a.partial_cmp(&(x + lk - 1e-9)).unwrap()).unwrap_or_else(|z| z);
        //    ans += j - i - 1;
        //}
        // This can replace the above for loop. It is faster.
        let mut m: usize = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        for i in 0..prefix.len() {
            lo = i + 1;
            hi = prefix.len();
            while lo < hi {
                m = lo + (hi - lo) / 2;
                if prefix[m] < prefix[i] + lk - 1e-9 {
                    lo = m + 1;
                } else {
                    hi = m;
                }
            }
            ans += lo - i - 1;
        }
        return ans as i32;
        */
    }
}