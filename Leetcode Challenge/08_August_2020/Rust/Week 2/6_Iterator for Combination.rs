//Design an Iterator class, which has:
//
//A constructor that takes a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
//A function next() that returns the next combination of length combinationLength in lexicographical order.
//A function hasNext() that returns True if and only if there exists a next combination.
// 
//
//Example:
//
//CombinationIterator iterator = new CombinationIterator("abc", 2); // creates the iterator.
//
//iterator.next(); // returns "ab"
//iterator.hasNext(); // returns true
//iterator.next(); // returns "ac"
//iterator.hasNext(); // returns true
//iterator.next(); // returns "bc"
//iterator.hasNext(); // returns false
// 
//
//Constraints:
//
//1 <= combinationLength <= characters.length <= 15
//There will be at most 10^4 function calls per test.
//It's guaranteed that all calls of the function next are valid.
//   Hide Hint #1  
//Generate all combinations as a preprocessing.
//   Hide Hint #2  
//Use bit masking to generate all the combinations.

use std::collections::BTreeSet;

struct CombinationIterator {
    combinations: Vec<String>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        let mut combi: BTreeSet<String> = BTreeSet::<String>::new();
        let mut characters: Vec<char> = characters.chars().collect();
        
        let mut total: u32 = (1 << characters.len()) - 1;
        for i in 0..total {
            let mut c: u32 = total & i;
            let mut position: usize = 0;
            let mut ones: i32 = 0;
            let mut cc: String = String::new();
            while ones < combinationLength && c != 0 {
                if 1 & c != 0 {
                    cc.push(characters[position]);
                    ones += 1;
                }
                c >>= 1;
                position += 1;
            }
            if ones == combinationLength {
                combi.insert(cc);
            }
        }
        let mut combinations: Vec<String> = combi.iter().cloned().collect();
        combinations.reverse();
        CombinationIterator{
            combinations: combinations
        }
    }
    
    fn next(&mut self) -> String {
        if CombinationIterator::has_next(&self) {
            let s: String = self.combinations.pop().expect("");
            s
        } else {
            String::new()
        }
    }
    
    fn has_next(&self) -> bool {
        !self.combinations.is_empty()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */