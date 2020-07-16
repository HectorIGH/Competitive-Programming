#You are given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point. Check if these points make a straight line in the XY plane.
#
# 
#
# 
#
#Example 1:
#
#
#
#Input: coordinates = [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]
#Output: true
#Example 2:
#
#
#
#Input: coordinates = [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]
#Output: false
# 
#
#Constraints:
#
#2 <= coordinates.length <= 1000
#coordinates[i].length == 2
#-10^4 <= coordinates[i][0], coordinates[i][1] <= 10^4
#coordinates contains no duplicate point.
#   Hide Hint #1  
#If there're only 2 points, return true.
#   Hide Hint #2  
#Check if all other points lie on the line defined by the first 2 points.
#   Hide Hint #3  
#Use cross product to check collinearity.


class Solution:
    def checkStraightLine(self, coordinates: List[List[int]]) -> bool:
        #dx = coordinates[1][0] - coordinates[0][0]
        #dy = coordinates[1][1] - coordinates[0][1]
        #for i in range(1, len(coordinates) - 1):
            #ndx = coordinates[i + 1][0] - coordinates[i][0]
            #ndy = coordinates[i + 1][1] - coordinates[i][1]
            #if dx * ndy - ndx * dy:
                #return False
        #return True
        ####
        #for a, b in zip(coordinates[2:], coordinates[1:-1]):
            #ndx = a[0] - b[0]
            #ndy = a[1] - b[1]
            #if dx * ndy - ndx * dy:
                #return False
        #return True
        ####
        #mapa = map(lambda a : dx*(a[0][1] - a[1][1]) - dy*(a[0][0] - a[1][0]), zip(coordinates[2:], coordinates[1:-1]))
        #return not any(mapa)
        for i in range(1, len(coordinates)):
            coordinates[i][0] -= coordinates[0][0]
            coordinates[i][1] -= coordinates[0][1]
        coordinates[0][0] = 0
        coordinates[0][1] = 0
        for i in range(len(coordinates) - 1):
            if coordinates[i][0] * coordinates[i+1][1] - coordinates[i][1]*coordinates[i+1][0]:
                return False
        return True