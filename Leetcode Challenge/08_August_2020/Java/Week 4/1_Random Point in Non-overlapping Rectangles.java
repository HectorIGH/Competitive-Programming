//Given a list of non-overlapping axis-aligned rectangles rects, write a function pick which randomly and uniformily picks an integer point in the space covered by the rectangles.
//
//Note:
//
//An integer point is a point that has integer coordinates. 
//A point on the perimeter of a rectangle is included in the space covered by the rectangles. 
//ith rectangle = rects[i] = [x1,y1,x2,y2], where [x1, y1] are the integer coordinates of the bottom-left corner, and [x2, y2] are the integer coordinates of the top-right corner.
//length and width of each rectangle does not exceed 2000.
//1 <= rects.length <= 100
//pick return a point as an array of integer coordinates [p_x, p_y]
//pick is called at most 10000 times.
//Example 1:
//
//Input: 
//["Solution","pick","pick","pick"]
//[[[[1,1,5,5]]],[],[],[]]
//Output: 
//[null,[4,1],[4,1],[3,3]]
//Example 2:
//
//Input: 
//["Solution","pick","pick","pick","pick","pick"]
//[[[[-2,-2,-1,-1],[1,0,3,0]]],[],[],[],[],[]]
//Output: 
//[null,[-1,-2],[2,0],[-2,-1],[3,0],[-2,-2]]
//Explanation of Input Syntax:
//
//The input is two lists: the subroutines called and their arguments. Solution's constructor has one argument, the array of rectangles rects. pick has no arguments. Arguments are always wrapped with a list, even if there aren't any.

class Solution {
    
    int[] w;
    int[][] rects;
    Random r;

    public Solution(int[][] rects) {
        w = new int[rects.length];
        this.rects = rects;
        r = new Random();
        int x1 = 0;
        int y1 = 0;
        int x2 = 0;
        int y2 = 0;
        for(int i = 0; i < rects.length; i++) {
            x1 = rects[i][0];
            y1 = rects[i][1];
            x2 = rects[i][2];
            y2 = rects[i][3];
            w[i] = (x2 - x1 + 1) * (y2 - y1 + 1);
        }
        for(int i = 1; i < rects.length; i++) {
            this.w[i] += w[i - 1];
        }
    }
    
    public int[] pick() {
        int target = 1 + r.nextInt(this.w[this.w.length - 1]);
        int n_rect = Arrays.binarySearch(this.w, target);
        if(n_rect < 0) {
            n_rect = ~n_rect;
        }
        int[] rect = this.rects[n_rect];
        int x1 = rect[0], x2 = rect[2], y1 = rect[1], y2 = rect[3];
        int x = r.nextInt(x2 - x1 + 1) + x1;
        int y = r.nextInt(y2 - y1 + 1) + y1;
        return new int[]{x, y};
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * Solution obj = new Solution(rects);
 * int[] param_1 = obj.pick();
 */