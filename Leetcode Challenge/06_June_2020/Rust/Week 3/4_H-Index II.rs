//Given an array of citations sorted in ascending order (each citation is a non-negative integer) of a researcher, write a function to compute the researcher's h-index.
//
//According to the definition of h-index on Wikipedia: "A scientist has index h if h of his/her N papers have at least h citations each, and the other N − h papers have no more than h citations each."
//
//Example:
//
//Input: citations = [0,1,3,5,6]
//Output: 3 
//Explanation: [0,1,3,5,6] means the researcher has 5 papers in total and each of them had 
//             received 0, 1, 3, 5, 6 citations respectively. 
//             Since the researcher has 3 papers with at least 3 citations each and the remaining 
//             two with no more than 3 citations each, her h-index is 3.
//Note:
//
//If there are several possible values for h, the maximum one is taken as the h-index.
//
//Follow up:
//
//This is a follow up problem to H-Index, where citations is now guaranteed to be sorted in ascending order.
//Could you solve it in logarithmic time complexity?
//   Hide Hint #1  
//Expected runtime complexity is in O(log n) and the input is sorted.

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if(citations.len() == 0) {
            return 0;
        }
        let n:i32 = citations.len() as i32;
        let mut l:i32 = 0;
        let mut r:i32 = n - 1;
        let mut mid:i32 = 0;
        let mut ans:i32 = 0;
        while l <= r {
            mid = l + (r - l) / 2;
            if citations[mid as usize] == n - mid {
                return n - mid;
            } else if citations[mid as usize] < n - mid {
                l = mid + 1;
            } else {
                ans = n - mid;
                r = mid - 1;
            }
        }
        ans
    }
}