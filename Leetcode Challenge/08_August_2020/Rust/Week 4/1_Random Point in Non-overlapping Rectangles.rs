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

use rand::{thread_rng, Rng};

struct Solution {
    w: Vec<i32>,
    rects: Vec<Vec<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut w: Vec<i32> = Vec::with_capacity(rects.len());
        let mut x1: &i32 = &0;
        let mut y1: &i32 = &0;
        let mut x2: &i32 = &0;
        let mut y2: &i32 = &0;
        for i in 0..rects.len() {
            x1 = &rects[i][0];
            y1 = &rects[i][1];
            x2 = &rects[i][2];
            y2 = &rects[i][3];
            w.push((x2 - x1 + 1) * (y2 - y1 + 1));
        }
        for i in 1..rects.len() {
            w[i] += w[i - 1];
        }
        Solution{
            w: w,
            rects: rects
        }
    }
    
    fn pick(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        
        let target: i32 = rng.gen_range(1, self.w.last().unwrap() + 1);
        let n_rect:usize = match self.w.binary_search(&target) {
            Ok(v) => v,
            Err(v) => v
        };
        
        let rect: &Vec<i32> = &self.rects[n_rect];
        let x1: &i32 = &rect[0];
        let x2: &i32 = &rect[2];
        let y1: &i32 = &rect[1];
        let y2: &i32 = &rect[3];
        let x: i32 = rng.gen_range(x1, x2 + 1);
        let y: i32 = rng.gen_range(y1, y2 + 1);
        vec![x, y]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(rects);
 * let ret_1: Vec<i32> = obj.pick();
 */