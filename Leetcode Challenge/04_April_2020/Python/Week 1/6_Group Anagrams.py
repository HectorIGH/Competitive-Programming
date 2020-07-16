#Given an array of strings, group anagrams together.
#
#Example:
#
#Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
#Output:
#[
#  ["ate","eat","tea"],
#  ["nat","tan"],
#  ["bat"]
#]
#Note:
#
#All inputs will be in lowercase.
#The order of your output does not matter.
def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
    lista = {}
    for s in strs:
        x = ''.join(sorted(s)) # Sorted the string
        if x not in lista:
            lista[x] = [s]
        else:
            lista[x].append(s)

    return lista.values()