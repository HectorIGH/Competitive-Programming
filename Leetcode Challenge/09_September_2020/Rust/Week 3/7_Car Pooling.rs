//You are driving a vehicle that has capacity empty seats initially available for passengers.  The vehicle only drives east (ie. it cannot turn around and drive west.)
//
//Given a list of trips, trip[i] = [num_passengers, start_location, end_location] contains information about the i-th trip: the number of passengers that must be picked up, and the locations to pick them up and drop them off.  The locations are given as the number of kilometers due east from your vehicle's initial location.
//
//Return true if and only if it is possible to pick up and drop off all passengers for all the given trips. 
//
// 
//
//Example 1:
//
//Input: trips = [[2,1,5],[3,3,7]], capacity = 4
//Output: false
//Example 2:
//
//Input: trips = [[2,1,5],[3,3,7]], capacity = 5
//Output: true
//Example 3:
//
//Input: trips = [[2,1,5],[3,5,7]], capacity = 3
//Output: true
//Example 4:
//
//Input: trips = [[3,2,7],[3,7,9],[8,3,9]], capacity = 11
//Output: true
// 
// 
//
//Constraints:
//
//trips.length <= 1000
//trips[i].length == 3
//1 <= trips[i][0] <= 100
//0 <= trips[i][1] < trips[i][2] <= 1000
//1 <= capacity <= 100000
//   Hide Hint #1  
//Sort the pickup and dropoff events by location, then process them in order.

use std::cmp::max;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut m: i32 = std::i32::MIN; //i32::MAX;
        for trip in &trips {
            m = max(trip[2], m);
        }
        let mut times:Vec<i32> = vec![0; m as usize + 2];
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        for trip in trips {
            i = trip[0];
            j = trip[1];
            k = trip[2];
            times[j as usize + 1] += i;
            times[k as usize + 1] -= i;
        }
        for p in 1..(m as usize + 2) {
            times[p] += times[p - 1];
        }
        *times.iter().max().unwrap() <= capacity
    }
}