#Given a directed, acyclic graph of N nodes.  Find all possible paths from node 0 to node N-1, and return them in any order.
#
#The graph is given as follows:  the nodes are 0, 1, ..., graph.length - 1.  graph[i] is a list of all nodes j for which the edge (i, j) exists.
#
#Example:
#Input: [[1,2], [3], [3], []] 
#Output: [[0,1,3],[0,2,3]] 
#Explanation: The graph looks like this:
#0--->1
#|    |
#v    v
#2--->3
#There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
#Note:
#
#The number of nodes in the graph will be in the range [2, 15].
#You can print different paths in any order, but you should keep the order of nodes inside one path.


class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        '''
        # BFS
        ans = []
        destiny = len(graph) - 1
        q = deque([[0]])
        while q:
            currentPath = q.pop()
            currentVertex = currentPath[-1]
            if currentVertex == destiny:
                ans.append(currentPath)
            for v in graph[currentVertex]:
                q.append(currentPath + [v])
        return ans
        '''
        # DFS
        self.ans = []
        self.dfs(graph, 0, len(graph) - 1, [])
        return self.ans
    
    def dfs(self, graph, source, destiny, currentPath):
        if source == destiny:
            self.ans.append(currentPath)
        currentPath.append(source)
        for v in graph[source]:
            copied = copy.deepcopy(currentPath)
            self.dfs(graph, v, destiny, copied)