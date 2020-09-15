//Given a set of non-overlapping intervals, insert a new interval into the intervals (merge if necessary).
//
//You may assume that the intervals were initially sorted according to their start times.
//
//Example 1:
//
//Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//Output: [[1,5],[6,9]]
//Example 2:
//
//Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//Output: [[1,2],[3,10],[12,16]]
//Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//NOTE: input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

use std::cmp::{min, max};

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut i: usize = 0;
        for (mut j, interval) in intervals.iter().enumerate() {
            i = j;
            if new_interval[0] > interval[1] {
                res.push(interval.to_vec());
            } else if new_interval[1] < interval[0] {
                j -= 1;
                i = j;
                break;
            } else {
                new_interval[0] = min(new_interval[0], interval[0]);
                new_interval[1] = max(new_interval[1], interval[1]);
            }
        }
        res.push(new_interval);
        if(i + 1 < intervals.len()) {
            res.append(&mut intervals[i as usize + 1..].to_vec());
        }
        return res; 
    }
}