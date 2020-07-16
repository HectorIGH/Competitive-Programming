#Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
#
#You have the following 3 operations permitted on a word:
#
#Insert a character
#Delete a character
#Replace a character
#Example 1:
#
#Input: word1 = "horse", word2 = "ros"
#Output: 3
#Explanation: 
#horse -> rorse (replace 'h' with 'r')
#rorse -> rose (remove 'r')
#rose -> ros (remove 'e')
#Example 2:
#
#Input: word1 = "intention", word2 = "execution"
#Output: 5
#Explanation: 
#intention -> inention (remove 't')
#inention -> enention (replace 'i' with 'e')
#enention -> exention (replace 'n' with 'x')
#exention -> exection (replace 'n' with 'c')
#exection -> execution (insert 'u')

class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        # Naive exponential solution
        '''
        if not word1:
            return len(word2)
        if not word2:
            return len(word1)
        def count(w1, w2):
            if not w1:
                return len(w2)
            if not w2:
                return len(w1)
            if w1[-1] == w2[-1]:
                return count(w1[:-1], w2[:-1])
            return 1 + min(count(w1, w2[:-1]), # Insert
                           count(w1[:-1], w2), # Remove
                           count(w1[:-1], w2[:-1])) # Replace
        return count(word1, word2)
        '''
        
        # DP solution time O(mxn) space O(mxn)
        m, n = len(word1), len(word2)
        dp = [[0 for x in range(n + 1)] for x in range(m + 1)] 
        # Fill d[][] in bottom up manner 
        for i in range(m + 1): 
            for j in range(n + 1): 
                # If first string is empty, only option is to 
                # insert all characters of second string 
                if i == 0: 
                    dp[i][j] = j    # Min. operations = j 

                # If second string is empty, only option is to 
                # remove all characters of second string 
                elif j == 0: 
                    dp[i][j] = i    # Min. operations = i 
                # If last characters are same, ignore last char 
                # and recur for remaining string 
                elif word1[i-1] == word2[j-1]: 
                    dp[i][j] = dp[i-1][j-1] 
                # If last character are different, consider all 
                # possibilities and find minimum 
                else:
                    dp[i][j] = 1 + min(dp[i][j-1],        # Insert
                                       dp[i-1][j],        # Remove
                                       dp[i-1][j-1])    # Replace
        return dp[m][n]