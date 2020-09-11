//You are given two images img1 and img2 both of size n x n, represented as binary, square matrices of the same size. (A binary matrix has only 0s and 1s as values.)
//
//We translate one image however we choose (sliding it left, right, up, or down any number of units), and place it on top of the other image.  After, the overlap of this translation is the number of positions that have a 1 in both images.
//
//(Note also that a translation does not include any kind of rotation.)
//
//What is the largest possible overlap?
//
// 
//
//Example 1:
//
//
//Input: img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
//Output: 3
//Explanation: We slide img1 to right by 1 unit and down by 1 unit.
//
//The number of positions that have a 1 in both images is 3. (Shown in red)
//
//Example 2:
//
//Input: img1 = [[1]], img2 = [[1]]
//Output: 1
//Example 3:
//
//Input: img1 = [[0]], img2 = [[0]]
//Output: 0
// 
//
//Constraints:
//
//n == img1.length
//n == img1[i].length
//n == img2.length
//n == img2[i].length
//1 <= n <= 30
//img1[i][j] is 0 or 1.
//img2[i][j] is 0 or 1.

class Solution {
    public int largestOverlap(int[][] img1, int[][] img2) {
        int n = img1.length;
        int max_overlaps = 0;
        for(int y_shift = 0; y_shift < n; y_shift++) {
            for(int x_shift = 0; x_shift < n; x_shift++) {
                max_overlaps = Math.max(max_overlaps, helper(img1, img2, x_shift, y_shift, n));
                max_overlaps = Math.max(max_overlaps, helper(img2, img1, x_shift, y_shift, n));
            }
        }
        return max_overlaps;
    }
    
    public int helper(int[][] A, int[][] B, int x_shift, int y_shift, int n) {
        int left_count = 0;
        int right_count = 0;
        for(int a_row = 0, b_row = y_shift; b_row < n; a_row++, b_row++) {
            for(int a_col = 0, b_col = x_shift; b_col < n; a_col++, b_col++) {
                if(A[a_row][a_col] == 1 && A[a_row][a_col] == B[b_row][b_col]) {
                    left_count += 1;
                }
                if(A[a_row][b_col] == 1 && A[a_row][b_col] == B[b_row][a_col]) {
                    right_count += 1;
                }
            }
        }
        return Math.max(left_count, right_count);
    }
}