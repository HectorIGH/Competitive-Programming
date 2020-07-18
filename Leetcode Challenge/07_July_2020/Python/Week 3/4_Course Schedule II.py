#There are a total of n courses you have to take, labeled from 0 to n-1.
#
#Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
#
#Given the total number of courses and a list of prerequisite pairs, return the ordering of courses you should take to finish all courses.
#
#There may be multiple correct orders, you just need to return one of them. If it is impossible to finish all courses, return an empty array.
#
#Example 1:
#
#Input: 2, [[1,0]] 
#Output: [0,1]
#Explanation: There are a total of 2 courses to take. To take course 1 you should have finished   
#             course 0. So the correct course order is [0,1] .
#Example 2:
#
#Input: 4, [[1,0],[2,0],[3,1],[3,2]]
#Output: [0,1,2,3] or [0,2,1,3]
#Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both     
#             courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0. 
#             So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3] .
#Note:
#
#The input prerequisites is a graph represented by a list of edges, not adjacency matrices. Read more about how a graph is represented.
#You may assume that there are no duplicate edges in the input prerequisites.
#   Hide Hint #1  
#This problem is equivalent to finding the topological order in a directed graph. If a cycle exists, no topological ordering exists and therefore it will be impossible to take all courses.
#   Hide Hint #2  
#Topological Sort via DFS - A great video tutorial (21 minutes) on Coursera explaining the basic concepts of Topological Sort.
#   Hide Hint #3  
#Topological sort could also be done via BFS.

class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        self.adj = defaultdict(list)
        self.ans = []
        self.visited = [0] * numCourses
        
        for u, v in prerequisites:
            self.adj[v].append(u)
        for i in range(numCourses):
            if self.visited[i] == 0 and self.dfs_cycle(i):
                return []
        return self.ans[::-1]
    
    def dfs_cycle(self, u):
        self.visited[u] = 1
        if u in self.adj:
            for v in self.adj[u]:
                if self.visited[v] == 1:
                    return True # Cycle found
                if self.visited[v] == 0 and self.dfs_cycle(v):
                    return True
        self.visited[u] = 2
        self.ans.append(u)
        return False