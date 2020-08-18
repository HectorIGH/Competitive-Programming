//Given a collection of intervals, find the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
//
// 
//
//Example 1:
//
//Input: [[1,2],[2,3],[3,4],[1,3]]
//Output: 1
//Explanation: [1,3] can be removed and the rest of intervals are non-overlapping.
//Example 2:
//
//Input: [[1,2],[1,2],[1,2]]
//Output: 2
//Explanation: You need to remove two [1,2] to make the rest of intervals non-overlapping.
//Example 3:
//
//Input: [[1,2],[2,3]]
//Output: 0
//Explanation: You don't need to remove any of the intervals since they're already non-overlapping.
// 
//
//Note:
//
//You may assume the interval's end point is always bigger than its start point.
//Intervals like [1,2] and [2,3] have borders "touching" but they don't overlap each other.

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            0
        } else {
            intervals.sort_unstable_by_key(|x| x[1]);
            let n: usize = intervals.len();
            let mut count: i32 = 1;
            let mut curr: &Vec<i32> = &intervals[0];
            for i in 0..n {
                if curr[1] <= intervals[i][0] {
                    count += 1;
                    curr = &intervals[i];
                }
            }
            n as i32 - count
        }
    }
}