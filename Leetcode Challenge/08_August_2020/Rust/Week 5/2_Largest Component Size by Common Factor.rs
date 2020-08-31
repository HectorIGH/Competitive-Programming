//Given a non-empty array of unique positive integers A, consider the following graph:
//
//There are A.length nodes, labelled A[0] to A[A.length - 1];
//There is an edge between A[i] and A[j] if and only if A[i] and A[j] share a common factor greater than 1.
//Return the size of the largest connected component in the graph.
//
// 
//
//Example 1:
//
//Input: [4,6,15,35]
//Output: 4
//
//Example 2:
//
//Input: [20,50,9,63]
//Output: 2
//
//Example 3:
//
//Input: [2,3,6,7,4,12,21,39]
//Output: 8
//
//Note:
//
//1 <= A.length <= 20000
//1 <= A[i] <= 100000

use std::collections::{HashSet, HashMap};
use std::cmp::max;

#[derive(Debug)]
struct DSU {
    p: Vec<i32>
}

impl DSU {
    
    fn new(N: i32) -> Self {
        let mut p: Vec<i32> = Vec::with_capacity(N as usize);
        for i in 0..N {
            p.push(i);
        }
        DSU {
            p: p
        }
    }
        
    fn find(&mut self, x: i32) -> i32 {
        if self.p[x as usize] != x {
            self.p[x as usize] = self.find(self.p[x as usize]);
        }
        self.p[x as usize]
    }
    
    fn union(&mut self, x: i32, y: i32) {
        let mut xr: i32 = self.find(x);
        let mut yr: i32 = self.find(y);
        self.p[xr as usize] = yr;
    }
}

impl Solution {
    pub fn largest_component_size(a: Vec<i32>) -> i32 {
        let mut n: i32 = a.len() as i32;
        let mut UF: DSU = DSU::new(n);
        let mut primes: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut num: i32 = 0;
        let mut prime_set: HashSet<i32> = HashSet::new();
        for i in 0..n {
            num = a[i as usize];
            prime_set = Solution::primes_set(num);
            for q in prime_set {
                let mut entry = primes.entry(q).or_insert(Vec::new());
                entry.push(i)
            }
        }
        for (_, indexes) in primes {
            for i in 0..(indexes.len() - 1) {
                UF.union(indexes[i], indexes[i + 1]);
            }
        }
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for i in 0..n {
            let look: i32 = UF.find(i);
            let counter = counts.entry(look).or_insert(0);
            *counter += 1;
        }
        let mut maxi: i32 = 0;
        for (_, value) in counts {
            maxi = max(maxi, value);
        }
        maxi
    }
    
    pub fn primes_set(n: i32) -> HashSet<i32> {
        let mut unique: HashSet<i32> = HashSet::new();
        let mut bound: i32 = (n as f64).sqrt() as i32 + 1;
        for i in 2..bound {
            if n % i == 0 {
                unique.insert(i);
                let mut t: HashSet<i32> = Solution::primes_set((n / i) as i32);
                for n in t {
                    unique.insert(n);
                }
                return unique;
            }
        }
        unique.insert(n);
        return unique;
    }
}