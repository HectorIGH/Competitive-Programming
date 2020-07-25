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


class Solution {
public:
    vector<vector<int>> allPathsSourceTarget(vector<vector<int>>& graph) {
        /*
        // BFS
        vector<vector<int>> ans;
        int destiny = graph.size() - 1;
        deque<vector<int>> q;
        q.push_back({0});
        while(!q.empty()) {
            vector<int> curPath = q.front();
            q.pop_front();
            int currentVertex = curPath[curPath.size() - 1];
            if(currentVertex == destiny) {
                ans.push_back(curPath);
            }
            for(int v : graph[currentVertex]) {
                vector<int> path(curPath);
                path.push_back(v);
                q.push_back(path);
            }
        }
        return ans;
        */
        vector<vector<int>> ans;
        vector<int> path;
        dfs(graph, 0, graph.size() - 1, path, ans);
        return ans;
    }
    void dfs(vector<vector<int>>& graph, int source, int destiny, vector<int>& path, vector<vector<int>>& ans) {
        path.push_back(source);
        if(source == destiny) {
            ans.push_back(path);
        }
        for(int v : graph[source]) {
            vector<int> copyPath(path);
            dfs(graph, v, destiny, copyPath, ans);
        }
    }
};