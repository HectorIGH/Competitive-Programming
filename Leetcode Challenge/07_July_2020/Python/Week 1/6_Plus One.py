#Given a non-empty array of digits representing a non-negative integer, increment one to the integer.
#
#The digits are stored such that the most significant digit is at the head of the list, and each element in the array contains a single digit.
#
#You may assume the integer does not contain any leading zero, except the number 0 itself.
#
#Example 1:
#
#Input: [1,2,3]
#Output: [1,2,4]
#Explanation: The array represents the integer 123.
#Example 2:
#
#Input: [4,3,2,1]
#Output: [4,3,2,2]
#Explanation: The array represents the integer 4321.

class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        # Only Python
        '''
        t = int("".join([str(i) for i in digits])) + 1
        return [int(i) for i in str(t)]
        '''
        digits[-1] += 1
        if digits[-1] != 10:
            return digits
        i = len(digits) - 1
        while i >= 0 and digits[i] == 10:
            digits[i] = 0
            if i == 0:
                digits.insert(0, 1)
                #digits = [1] + digits
            else:
                digits[i - 1] += 1
            i -= 1
        return digits