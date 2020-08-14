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

class CombinationIterator {
public:
    
    set<string> combinations;
    
    CombinationIterator(string characters, int combinationLength) {
        int total = (1 << characters.size()) - 1;
        for(int i = 0; i < total; i++) {
            int c = total & i;
            int position = 0;
            int ones = 0;
            string cc;
            while(ones < combinationLength && c != 0) {
                if((1 & c) != 0) {
                    cc.push_back(characters[position]);
                    ones++;
                }
                c >>= 1;
                position++;
            }
            if(ones == combinationLength) {
                combinations.insert(cc);
            }
        }
    }
    
    string next() {
        if(hasNext()) {
            set<string>::iterator it = combinations.begin();
            string s = *it;
            combinations.erase(it);
            return s;
        }
        return nullptr;
    }
    
    bool hasNext() {
        return !combinations.empty();
    }
};

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * CombinationIterator* obj = new CombinationIterator(characters, combinationLength);
 * string param_1 = obj->next();
 * bool param_2 = obj->hasNext();
 */