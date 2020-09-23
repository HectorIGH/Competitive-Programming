//Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.
//
//Note: The algorithm should run in linear time and in O(1) space.
//
//Example 1:
//
//Input: [3,2,3]
//Output: [3]
//Example 2:
//
//Input: [1,1,1,3,3,2,2,2]
//Output: [1,2]
//   Hide Hint #1  
//How many majority elements could it possibly have?
//Do you have a better hint? Suggest it!

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut l: usize = nums.len();
        let mut result: Vec<i32> = Vec::new();
        if l == 0 {
            return result;
        }
        let mut count1: u8 = 0;
        let mut count2: u8 = 0;
        let mut candidate1: Option::<i32> = Option::<i32>::None;
        let mut candidate2: Option::<i32> = Option::<i32>::None;
        for n in &nums {
            if candidate1.is_some() && candidate1.unwrap() == *n {
                count1 += 1;
            } else if candidate2.is_some() && candidate2.unwrap() == *n {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = Some(*n);
                count1 += 1;
            } else if count2 == 0 {
                candidate2 = Some(*n);
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        
        if let Some(c1) = candidate1 {
            if nums.iter().filter(|&x| *x == c1).count() > l / 3 {
                result.push(c1);
            }
        }
        if let Some(c2) = candidate2 {
            if nums.iter().filter(|&x| *x == c2).count() > l / 3 {
                result.push(c2);
            }
        }
        //if candidate1.is_some() && nums.iter().filter(|&x| *x == candidate1.unwrap()).count() > l / 3 {
        //        result.push(candidate1.unwrap());
        //}
        //if candidate2.is_some() && nums.iter().filter(|&x| *x == candidate2.unwrap()).count() > l / 3 {
        //        result.push(candidate2.unwrap());
        //}
                
        return result;
    }
}