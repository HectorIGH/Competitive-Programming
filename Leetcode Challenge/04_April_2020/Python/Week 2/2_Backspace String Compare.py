#Given two strings S and T, return if they are equal when both are typed into empty text editors. # means a backspace character.
#
#Example 1:
#
#Input: S = "ab#c", T = "ad#c"
#Output: true
#Explanation: Both S and T become "ac".
#Example 2:
#
#Input: S = "ab##", T = "c#d#"
#Output: true
#Explanation: Both S and T become "".
#Example 3:
#
#Input: S = "a##c", T = "#a#c"
#Output: true
#Explanation: Both S and T become "c".
#Example 4:
#
#Input: S = "a#c", T = "b"
#Output: false
#Explanation: S becomes "c" while T becomes "b".
#Note:
#
#1 <= S.length <= 200
#1 <= T.length <= 200
#S and T only contain lowercase letters and '#' characters.
#Follow up:
#
#Can you solve it in O(N) time and O(1) space?

def backspaceCompare(self, S: str, T: str) -> bool:
    #S = [l for l in S]

    #for i in range(len(S)):
    #    if S[i] == '#': # Find the first non - # previous and turn it into #
    #        for j in range(i, -1, -1):
    #            if S[j] != '#':
    #                S[j] = '#'
    #                break
    #ind = 0
    #for e in S:
    #    if e != '#':
    #        S[ind] = e
    #        ind += 1
    #for i in range(ind, len(S)):
    #    #S[i] = '#'
    #    S.pop(-1)
    ################
    #T = [l for l in T]

    #for i in range(len(T)):
    #    if T[i] == '#': # Find the first non - # previous and turn it into #
    #        for j in range(i, -1, -1):
    #            if T[j] != '#':
    #                T[j] = '#'
    #                break
    #ind = 0
    #for e in T:
    #    if e != '#':
    #        T[ind] = e
    #        ind += 1
    #for i in range(ind, len(T)):
    #    #S[i] = '#'
    #    T.pop(-1)

    #if len(S) == len(T):
    #    for i in range(len(S)):
    #        if S[i] != T[i]:
    #            return False
    #    return True
    #return False

    # Second and better solution
    
    def foo(S:str):
        stack = []
        for l in S:
            if stack and l == '#':
                stack.pop()
            elif l != '#':
                stack.append(l)
        return stack
    return foo(S) == foo(T)