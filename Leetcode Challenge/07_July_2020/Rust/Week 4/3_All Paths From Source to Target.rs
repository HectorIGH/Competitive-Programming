//Given a directed, acyclic graph of N nodes.  Find all possible paths from node 0 to node N-1, and return them in any order.
//
//The graph is given as follows:  the nodes are 0, 1, ..., graph.length - 1.  graph[i] is a list of all nodes j for which the edge (i, j) exists.
//
//Example:
//Input: [[1,2], [3], [3], []] 
//Output: [[0,1,3],[0,2,3]] 
//Explanation: The graph looks like this:
//0--->1
//|    |
//v    v
//2--->3
//There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
//Note:
//
//The number of nodes in the graph will be in the range [2, 15].
//You can print different paths in any order, but you should keep the order of nodes inside one path.


use std::collections::VecDeque;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /*
        // BFS
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut destiny : i32 = (graph.len() - 1) as i32;
        let mut q : VecDeque<Vec<i32>> = VecDeque::new();
        q.push_back(vec![0]);
        while !q.is_empty() {
            if let Some(cur_path) = q.pop_front() {
                let mut currentVertex = cur_path[cur_path.len() - 1];
                if currentVertex == destiny {
                    ans.push(cur_path.clone());
                }
                for v in &graph[currentVertex as usize] {
                    let mut path = cur_path.clone();
                    path.push(*v);
                    q.push_back(path);
                }
            }
        }
        return ans;
        */
        let mut ans : Vec<Vec<i32>> = Vec::new();
        let mut path : Vec<i32> = Vec::new();
        Solution::dfs(&graph, 0, graph.len() - 1, &mut path, &mut  ans);
        ans
    }
    pub fn dfs(graph : &Vec<Vec<i32>>, source : usize, destiny : usize, path : &mut Vec<i32>, ans : &mut Vec<Vec<i32>>) {
        path.push(source as i32);
        if source == destiny {
            ans.push(path.to_vec());
        }
        for v in &graph[source] {
            let mut copyPath = path.clone();
            Solution::dfs(graph, *v as usize, destiny, &mut copyPath, ans);
        }
    }
}