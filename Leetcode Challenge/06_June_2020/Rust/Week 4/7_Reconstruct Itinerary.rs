//Given a list of airline tickets represented by pairs of departure and arrival airports [from, to], reconstruct the itinerary in order. All of the tickets belong to a man who departs from JFK. Thus, the itinerary must begin with JFK.
//
//Note:
//
//If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string. For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
//All airports are represented by three capital letters (IATA code).
//You may assume all tickets form at least one valid itinerary.
//One must use all the tickets once and only once.
//Example 1:
//
//Input: [["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]]
//Output: ["JFK", "MUC", "LHR", "SFO", "SJC"]
//Example 2:
//
//Input: [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
//Output: ["JFK","ATL","JFK","SFO","ATL","SFO"]
//Explanation: Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"].
//             But it is larger in lexical order.

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut flights = HashMap::<&str, BinaryHeap<Reverse<&str>>>::new();
        let mut path : Vec<String> = vec![];
        let mut stack = vec!["JFK"];
        
        for ticket in tickets.iter() {
            flights.entry(&ticket[0])
            .or_insert_with(BinaryHeap::new)
            .push(Reverse(&ticket[1]));
        }
        
        while let Some(current) = stack.last() {
            if let Some(tos) = flights.get_mut(&current.clone()) {
                if !tos.is_empty() {
                    if let Some(to) = tos.pop() {
                        stack.push(to.0);
                    }
                    continue;
                }
            }
            if let Some(last) = stack.pop() {
                path.push(last.to_string());
            }
        }
        path.reverse();
        return path;
    }
}