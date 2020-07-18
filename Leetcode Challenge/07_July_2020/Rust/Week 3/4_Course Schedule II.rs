//There are a total of n courses you have to take, labeled from 0 to n-1.
//
//Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
//
//Given the total number of courses and a list of prerequisite pairs, return the ordering of courses you should take to finish all courses.
//
//There may be multiple correct orders, you just need to return one of them. If it is impossible to finish all courses, return an empty array.
//
//Example 1:
//
//Input: 2, [[1,0]] 
//Output: [0,1]
//Explanation: There are a total of 2 courses to take. To take course 1 you should have finished   
//             course 0. So the correct course order is [0,1] .
//Example 2:
//
//Input: 4, [[1,0],[2,0],[3,1],[3,2]]
//Output: [0,1,2,3] or [0,2,1,3]
//Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both     
//             courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0. 
//             So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3] .
//Note:
//
//The input prerequisites is a graph represented by a list of edges, not adjacency matrices. Read more about how a graph is represented.
//You may assume that there are no duplicate edges in the input prerequisites.
//   Hide Hint #1  
//This problem is equivalent to finding the topological order in a directed graph. If a cycle exists, no topological ordering exists and therefore it will be impossible to take all courses.
//   Hide Hint #2  
//Topological Sort via DFS - A great video tutorial (21 minutes) on Coursera explaining the basic concepts of Topological Sort.
//   Hide Hint #3  
//Topological sort could also be done via BFS.

use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adj : HashMap<usize, Vec<usize>> = HashMap::new();
        let mut ans : Vec<i32> = Vec::new();
        let mut visited : Vec<u8> = vec![0; num_courses as usize];
        
        for c in &prerequisites {
            adj.entry(c[1] as usize).or_insert(Vec::new()).push(c[0] as usize);
        }
        for i in 0..num_courses as usize {
            if visited[i] == 0 && Solution::dfs_cycle(i, &mut adj, &mut ans, &mut visited) {
                return Vec::new();
            }
        }
        ans.reverse();
        return ans;
    }
    
    pub fn dfs_cycle(u : usize, adj : &mut HashMap<usize, Vec<usize>>, ans : &mut Vec<i32>, visited : &mut Vec<u8>) -> bool {
        visited[u] = 1;
        if adj.contains_key(&u) {
            for v in adj.get_mut(&u).expect("Failed to retrieve adjacency list").clone() {
                if visited[v as usize] == 1 {
                    return true;
                }
                if visited[v as usize] == 0 && Solution::dfs_cycle(v, adj, ans, visited) {
                    return true;
                }
            }
        }
        visited[u] = 2;
        ans.push(u as i32);
        return false;
    }
}