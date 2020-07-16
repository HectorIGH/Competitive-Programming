//Design a data structure that supports all following operations in average O(1) time.
//
// 
//
//insert(val): Inserts an item val to the set if not already present.
//remove(val): Removes an item val from the set if present.
//getRandom: Returns a random element from current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
// 
//
//Example:
//
//// Init an empty set.
//RandomizedSet randomSet = new RandomizedSet();
//
//// Inserts 1 to the set. Returns true as 1 was inserted successfully.
//randomSet.insert(1);
//
//// Returns false as 2 does not exist in the set.
//randomSet.remove(2);
//
//// Inserts 2 to the set, returns true. Set now contains [1,2].
//randomSet.insert(2);
//
//// getRandom should return either 1 or 2 randomly.
//randomSet.getRandom();
//
//// Removes 1 from the set, returns true. Set now contains [2].
//randomSet.remove(1);
//
//// 2 was already in the set, so return false.
//randomSet.insert(2);
//
//// Since 2 is the only number in the set, getRandom always return 2.
//randomSet.getRandom();

use std::collections::HashMap;
use rand::prelude::*;

struct RandomizedSet {
    indexes: HashMap<i32, i32>,
    values: Vec<i32>,
    rng: ThreadRng,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        RandomizedSet {
            indexes : HashMap::new(),
            values : vec![],
            rng: rand::thread_rng(),
        }
    }
    
    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.indexes.contains_key(&val) {
            return false;
        }
        self.values.push(val);
        self.indexes.insert(val, (self.values.len() - 1) as i32);
        return true;
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        return match self.indexes.get_mut(&val) {
            None => {false},
            Some(&mut i) => {
                let last = self.values[self.values.len() - 1];
                self.values[i as usize] = last;
                self.indexes.insert(last, i);
                self.indexes.remove(&val);
                self.values.pop();
                true
            }
        }
    }
    
    /** Get a random element from the set. */
    fn get_random(&mut self) -> i32 {
        self.values[self.rng.gen_range(0, self.values.len())]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */