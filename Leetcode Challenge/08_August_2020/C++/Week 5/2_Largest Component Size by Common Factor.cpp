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

class DSU {
public:
    vector<int> p;
    
    DSU(int N) {
        for(int i = 0; i < N; i++) {
            p.push_back(i);
        }
    }
        
    int find(int x) {
        if(p[x] != x) {
            p[x] = find(p[x]);
        }
        return p[x];
    }
    
    void onion(int x, int y) {
        int xr = find(x);
        int yr = find(y);
        p[xr] = yr;
    }
};

class Solution {
public:
    int largestComponentSize(vector<int>& A) {
        int n = A.size();
        DSU UF(n);
        unordered_map<int, vector<int>> primes;
        int num;
        unordered_set<int> prime_set;
        for(int i = 0; i < n; i++) {
            num = A[i];
            prime_set = primes_set(num);
            for(int q : prime_set) {
                if(primes.count(q)) { // has key
                    primes[q].push_back(i);
                } else {
                    primes[q] = {i};
                }
            }
        }
        
        for(auto kv : primes) {
            vector<int> indexes = kv.second;
            for(int i = 0; i < indexes.size() - 1; i++) {
                UF.onion(indexes[i], indexes[i + 1]);
            }
        }
        map<int, int> counts;
        for(int i = 0; i < n; i++) {
            int look = UF.find(i);
            if(counts.count(look)) {
                counts[look]++;
            } else {
                counts[look] = 1;
            }
        }
        int maxi = 0;
        for(auto kv : counts) {
            maxi = max(maxi, kv.second);
        }
        return maxi; //max_element(counts.begin(), counts.end())->first;
    }
    
    unordered_set<int> primes_set(int n) {
        unordered_set<int> unique;
        for(int i = 2; i < (int)sqrt(n) + 1; i++) {
            if(n % i == 0) {
                //unique = new HashSet<>();
                unique.insert(i);
                auto t = primes_set((int)(n / i));
                unique.insert(t.begin(), t.end());
                return unique;
            }
        }
        unique.insert(n);
        return unique;
    }
};