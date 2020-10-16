//Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
//
//Integers in each row are sorted from left to right.
//The first integer of each row is greater than the last integer of the previous row.
// 
//
//Example 1:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,50]], target = 3
//Output: true
//Example 2:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,50]], target = 13
//Output: false
//Example 3:
//
//Input: matrix = [], target = 0
//Output: false
// 
//
//Constraints:
//
//m == matrix.length
//n == matrix[i].length
//0 <= m, n <= 100
//-104 <= matrix[i][j], target <= 104

class Solution {
    public boolean searchMatrix(int[][] matrix, int target) {
        if(matrix.length == 0 || matrix[0].length == 0) {
            return false;
        }
        int m = matrix.length;
        int n = matrix[0].length;
        int lo = 0;
        int hi = m * n - 1;
        int mid = 0;
        while(lo < hi) {
            mid = lo + (hi - lo) / 2;
            if(matrix[mid / n][mid % n] < target) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return matrix[lo / n][lo % n] == target;
    }
}