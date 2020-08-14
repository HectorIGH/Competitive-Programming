#Design an Iterator class, which has:
#
#A constructor that takes a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
#A function next() that returns the next combination of length combinationLength in lexicographical order.
#A function hasNext() that returns True if and only if there exists a next combination.
# 
#
#Example:
#
#CombinationIterator iterator = new CombinationIterator("abc", 2); // creates the iterator.
#
#iterator.next(); // returns "ab"
#iterator.hasNext(); // returns true
#iterator.next(); // returns "ac"
#iterator.hasNext(); // returns true
#iterator.next(); // returns "bc"
#iterator.hasNext(); // returns false
# 
#
#Constraints:
#
#1 <= combinationLength <= characters.length <= 15
#There will be at most 10^4 function calls per test.
#It's guaranteed that all calls of the function next are valid.
#   Hide Hint #1  
#Generate all combinations as a preprocessing.
#   Hide Hint #2  
#Use bit masking to generate all the combinations.

class CombinationIterator:

    def __init__(self, characters: str, combinationLength: int):
        self.combis = set()
        #self.comb = OrderedDict()
        #self.still = math.comb(len(characters), combinationLength)
        total = (1 << len(characters)) - 1
        for i in range(total):
            c = total & i # iterate as binary 10101001
            position = 0
            ones = 0
            cc = ''
            while ones < combinationLength and c:
                if 1 & c: # c has 1 in current position
                    cc += characters[position]
                    ones += 1
                c >>= 1
                position += 1
            if ones == combinationLength:
                self.combis.add(cc)
                #self.comb[cc] = None
        self.combinations = sorted(list(self.combis), reverse = True)
        #self.combinations = list(sorted(self.comb.keys(), reverse = True))

    def next(self) -> str:
        if self.hasNext():
            return self.combinations.pop()

    def hasNext(self) -> bool:
        return len(self.combinations)


# Your CombinationIterator object will be instantiated and called as such:
# obj = CombinationIterator(characters, combinationLength)
# param_1 = obj.next()
# param_2 = obj.hasNext()