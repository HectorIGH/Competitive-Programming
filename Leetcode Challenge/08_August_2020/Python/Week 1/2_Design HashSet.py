#Design a HashSet without using any built-in hash table libraries.
#
#To be specific, your design should include these functions:
#
#add(value): Insert a value into the HashSet. 
#contains(value) : Return whether the value exists in the HashSet or not.
#remove(value): Remove a value in the HashSet. If the value does not exist in the HashSet, do nothing.
#
#Example:
#
#MyHashSet hashSet = new MyHashSet();
#hashSet.add(1);         
#hashSet.add(2);         
#hashSet.contains(1);    // returns true
#hashSet.contains(3);    // returns false (not found)
#hashSet.add(2);          
#hashSet.contains(2);    // returns true
#hashSet.remove(2);          
#hashSet.contains(2);    // returns false (already removed)
#
#Note:
#
#All values will be in the range of [0, 1000000].
#The number of operations will be in the range of [1, 10000].
#Please do not use the built-in HashSet library.

class MyHashSet:
    
    def get_hash(self, key) -> int:
        # hash_a(K) = aK mod 2^w / 2^(w-m)
        # w = 20, m = 15, a = 1031237
        # w : wordsize, m : bitlength of hash, a : prime
        # for any s: s % (2^t) = s & (1<<t) - 1.
        return ((key * 1031237) & (1 << 20) - 1) >> 5

    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.arr = [[] for _ in range(1<<15)]
        
    def add(self, key: int) -> None:
        i = self.get_hash(key)
        if key not in self.arr[i]:
            self.arr[i].append(key)

    def remove(self, key: int) -> None:
        i = self.get_hash(key)
        if key in self.arr[i]:
            self.arr[i].remove(key)

    def contains(self, key: int) -> bool:
        """
        Returns true if this set contains the specified element
        """
        i = self.get_hash(key)
        return key in self.arr[i]
        


# Your MyHashSet object will be instantiated and called as such:
# obj = MyHashSet()
# obj.add(key)
# obj.remove(key)
# param_3 = obj.contains(key)