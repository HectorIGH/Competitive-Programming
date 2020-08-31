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
    int[] p;
    
    public DSU(int N) {
        p = new int[N];
        for(int i = 0; i < N; i++) {
            p[i] = i;
        }
    }
        
    public int find(int x) {
        if(p[x] != x) {
            p[x] = find(p[x]);
        }
        return p[x];
    }
    
    public void union(int x, int y) {
        int xr = find(x);
        int yr = find(y);
        p[xr] = yr;
    }
}

class Solution {
    public int largestComponentSize(int[] A) {
        int n = A.length;
        DSU UF = new DSU(n);
        HashMap<Integer, ArrayList<Integer>> primes = new HashMap<>();
        int num;
        HashSet<Integer> prime_set;
        for(int i = 0; i < n; i++) {
            num = A[i];
            prime_set = primes_set(num);
            for(Integer q : prime_set) {
                ArrayList<Integer> temp = primes.getOrDefault(q, new ArrayList<Integer>());
                temp.add(i);
                primes.put(q, temp);
            }
        }
                    
        for(ArrayList<Integer> indexes : primes.values()) {
            for(int i = 0; i < indexes.size() - 1; i++) {
                UF.union(indexes.get(i), indexes.get(i + 1));
            }
        }
        HashMap<Integer, Integer> counts = new HashMap<>();
        for(int i = 0; i < n; i++) {
            int look = UF.find(i);
            int count = counts.getOrDefault(look, 0);
            counts.put(look, count + 1);
        }
         return Collections.max(counts.values());
    }
    
    public HashSet<Integer> primes_set(int n) {
        HashSet<Integer> unique;
        for(int i = 2; i < (int)Math.sqrt(n) + 1; i++) {
            if(n % i == 0) {
                unique = new HashSet<>();
                unique.add(i);
                unique.addAll(primes_set((int)(n / i)));
                return unique;
            }
        }
        unique = new HashSet<>();
        unique.add(n);
        return unique;
    }
}