//Given a list of intervals, remove all intervals that are covered by another interval in the list.
//
//Interval [a,b) is covered by interval [c,d) if and only if c <= a and b <= d.
//
//After doing so, return the number of remaining intervals.
//
// 
//
//Example 1:
//
//Input: intervals = [[1,4],[3,6],[2,8]]
//Output: 2
//Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
//Example 2:
//
//Input: intervals = [[1,4],[2,3]]
//Output: 1
//Example 3:
//
//Input: intervals = [[0,10],[5,12]]
//Output: 2
//Example 4:
//
//Input: intervals = [[3,10],[4,10],[5,11]]
//Output: 2
//Example 5:
//
//Input: intervals = [[1,2],[1,4],[3,4]]
//Output: 1
// 
//
//Constraints:
//
//1 <= intervals.length <= 1000
//intervals[i].length == 2
//0 <= intervals[i][0] < intervals[i][1] <= 10^5
//All the intervals are unique.
//   Hide Hint #1  
//How to check if an interval is covered by another?
//   Hide Hint #2  
//Compare each interval to all others and check if it is covered by any interval.

impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|x: &Vec<i32>, y: &Vec<i32>| if x[0] != y[0] {x[0].cmp(&y[0])} else {y[1].cmp(&x[1])});
        let mut r: i32 = 0;
        let mut rem: i32 = 0;
        for interval in &intervals {
            if interval[1] <= r {
                rem += 1;
            } else {
                r = interval[1];
            }
        }
        return intervals.len() as i32 - rem;
    }
}