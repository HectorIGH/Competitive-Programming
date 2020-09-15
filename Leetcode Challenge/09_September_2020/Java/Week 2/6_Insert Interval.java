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

class Solution {
    public int[][] insert(int[][] intervals, int[] newInterval) {
        ArrayList<int[]> res = new ArrayList<>();
        int i = -1;
        int[] interval = new int[2];
        for(i = 0; i < intervals.length; i++) {
            interval = intervals[i];
            if(newInterval[0] > interval[1]) {
                res.add(interval);
            } else if(newInterval[1] < interval[0]) {
                i -= 1;
                break;
            } else {
                newInterval[0] = Math.min(newInterval[0], interval[0]);
                newInterval[1] = Math.max(newInterval[1], interval[1]);
            }
        }
        res.add(newInterval);
        for(int j = i + 1; j < intervals.length; j++) {
            res.add(intervals[j]);
        }
        
        int[][] arr = new int[res.size()][2];
        for(int j = 0; j < res.size(); j ++) {
            arr[j] = res.get(j);
        }
        return arr;
    }
}