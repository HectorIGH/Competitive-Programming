#You are given two images img1 and img2 both of size n x n, represented as binary, square matrices of the same size. (A binary matrix has only 0s and 1s as values.)
#
#We translate one image however we choose (sliding it left, right, up, or down any number of units), and place it on top of the other image.  After, the overlap of this translation is the number of positions that have a 1 in both images.
#
#(Note also that a translation does not include any kind of rotation.)
#
#What is the largest possible overlap?
#
# 
#
#Example 1:
#
#
#Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
#Output: 3
#Explanation: We slide img1 to right by 1 unit and down by 1 unit.
#
#The number of positions that have a 1 in both images is 3. (Shown in red)
#
#Example 2:
#
#Input: img1 = [[1]], img2 = [[1]]
#Output: 1
#Example 3:
#
#Input: img1 = [[0]], img2 = [[0]]
#Output: 0
# 
#
#Constraints:
#
#n == img1.length
#n == img1[i].length
#n == img2.length
#n == img2[i].length
#1 <= n <= 30
#img1[i][j] is 0 or 1.
#img2[i][j] is 0 or 1.

class Solution:
    def largestOverlap(self, img1: List[List[int]], img2: List[List[int]]) -> int:
        n = len(img1)
        max_overlaps = 0
        for y_shift in range(n):
            for x_shift in range(n):
                max_overlaps = max(max_overlaps, self.helper(img1, img2, x_shift, y_shift, n))
                max_overlaps = max(max_overlaps, self.helper(img2, img1, x_shift, y_shift, n))
        return max_overlaps
    
    def helper(self, A, B, x_shift, y_shift, n):
        left_count = 0
        right_count = 0
        for a_row, b_row in enumerate(range(y_shift, n)):
            for a_col, b_col in enumerate(range(x_shift, n)):
                if A[a_row][a_col] == 1 and A[a_row][a_col] == B[b_row][b_col]:
                    left_count += 1
                if A[a_row][b_col] == 1 and A[a_row][b_col] == B[b_row][a_col]:
                    right_count += 1
        
        return max(left_count, right_count)