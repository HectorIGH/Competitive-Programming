//There are n cities connected by m flights. Each flight starts from city u and arrives at v with a price w.
//
//Now given all the cities and flights, together with starting city src and the destination dst, your task is to find the cheapest price from src to dst with up to k stops. If there is no such route, output -1.
//
//Example 1:
//Input: 
//n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
//src = 0, dst = 2, k = 1
//Output: 200
//Explanation: 
//The graph looks like this:
//
//
//The cheapest price from city 0 to city 2 with at most 1 stop costs 200, as marked red in the picture.
//Example 2:
//Input: 
//n = 3, edges = [[0,1,100],[1,2,100],[0,2,500]]
//src = 0, dst = 2, k = 0
//Output: 500
//Explanation: 
//The graph looks like this:
//
//
//The cheapest price from city 0 to city 2 with at most 0 stop costs 500, as marked blue in the picture.
// 
//
//Constraints:
//
//The number of nodes n will be in range [1, 100], with nodes labeled from 0 to n - 1.
//The size of flights will be in range [0, n * (n - 1) / 2].
//The format of each flight will be (src, dst, price).
//The price of each flight will be in the range [1, 10000].
//k is in the range of [0, n - 1].
//There will not be any duplicated flights or self cycles.


use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut min_price : i32 = i32::max_value();
        let mut graph = HashMap::<i32, Vec<(i32, i32)>>::new();
        let mut q = VecDeque::new();
        q.push_back((src, 0, 0));
        for flight in &flights {
            match graph.get_mut(&flight[0]) {
                Some(v) => {v.push((flight[1], flight[2]));}
                None => {graph.insert(flight[0], vec![(flight[1], flight[2])]);}
            }
        }
        while let Some((city, visited, price)) = q.pop_front() {
            if price <= min_price && visited <= k && city != dst {
                match graph.get_mut(&city) {
                    None => {continue;}
                    Some(f) => {
                        for (neighbor, neighbor_price) in f {
                            q.push_back((*neighbor, visited + 1, price + *neighbor_price));
                        }
                    }
                }
            }
            if city == dst {
                min_price = cmp::min(min_price, price);
            }
        }
        return if min_price == i32::max_value() {-1} else {min_price};
    }
}