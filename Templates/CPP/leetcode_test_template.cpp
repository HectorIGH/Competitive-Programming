#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <sstream>
#include <queue>
#include <deque>
#include <bitset>
#include <iterator>
#include <list>
#include <stack>
#include <map>
#include <set>
#include <functional>
#include <numeric>
#include <utility>
#include <limits>
#include <time.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <assert.h>

using namespace std;

class Solution {
public:
    int change(int amount, vector<int>& coins) {
        int lc = coins.size();
        vector<int> dp(amount + 1, 0);
        dp[0] = 1;
        for(int i = 0; i < lc; i++){
            for(int j = coins[i]; j <= amount; j++){
                dp[j] += dp[j - coins[i]];
            }
        }
        return dp[amount];
    }
};

int main() {
    Solution s;
    vector<int> c = {1,2,5};
    cout << s.change(5, c);
}