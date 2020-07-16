#Write a function to check whether an input string is a valid IPv4 address or IPv6 address or neither.
#
#IPv4 addresses are canonically represented in dot-decimal notation, which consists of four decimal numbers, each ranging from 0 to 255, separated by dots ("."), e.g.,172.16.254.1;
#
#Besides, leading zeros in the IPv4 is invalid. For example, the address 172.16.254.01 is invalid.
#
#IPv6 addresses are represented as eight groups of four hexadecimal digits, each group representing 16 bits. The groups are separated by colons (":"). For example, the address 2001:0db8:85a3:0000:0000:8a2e:0370:7334 is a valid one. Also, we could omit some leading zeros among four hexadecimal digits and some low-case characters in the address to upper-case ones, so 2001:db8:85a3:0:0:8A2E:0370:7334 is also a valid IPv6 address(Omit leading zeros and using upper cases).
#
#However, we don't replace a consecutive group of zero value with a single empty group using two consecutive colons (::) to pursue simplicity. For example, 2001:0db8:85a3::8A2E:0370:7334 is an invalid IPv6 address.
#
#Besides, extra leading zeros in the IPv6 is also invalid. For example, the address 02001:0db8:85a3:0000:0000:8a2e:0370:7334 is invalid.
#
# 
#
#Example 1:
#
#Input: IP = "172.16.254.1"
#Output: "IPv4"
#Explanation: This is a valid IPv4 address, return "IPv4".
#Example 2:
#
#Input: IP = "2001:0db8:85a3:0:0:8A2E:0370:7334"
#Output: "IPv6"
#Explanation: This is a valid IPv6 address, return "IPv6".
#Example 3:
#
#Input: IP = "256.256.256.256"
#Output: "Neither"
#Explanation: This is neither a IPv4 address nor a IPv6 address.
# 
#
#Constraints:
#
#IP consists only of English letters, digits and the characters "." and ":".

class Solution:
    def validIPAddress(self, IP: str) -> str:
        n = len(IP)
        ans = "Neither"
        if n < 7 or n > 39:
            return ans
        # Regex
        '''
        if IP.count('.') == 3:
            for n in IP.split('.'):
                value = re.sub(r'[^0-9]', '', n)
                if not value or not str(int(value)) == n or not int(n) in range(256):
                    return ans
            ans = 'IPv4'
        elif IP.count(':') == 7:
            for n in IP.split(':'):
                value = re.sub(r'[^0-9a-fA-F]', '', n)
                if not value or not value == n or len(value) > 4 or int(value, 16) < 0:
                    return ans
            ans = 'IPv6'        
        return ans
        '''
        if IP.count('.') == 3:
            parts = IP.split('.')
            for i, n in enumerate(parts):
                if len(n) < 1:
                    return ans
                if len(n) > 1 and n[0] == '0' or n[0] == '-':
                    parts[i] = False
                    break
                try:
                    value = int(n)
                    parts[i] = (0 <= value < 1<<8)
                except:
                    parts[i] = False
                    break
            ans = 'IPv4' if all(parts) else ans
        elif IP.count(':') == 7:
            parts = IP.split(':')
            for i, n in enumerate(parts):
                if len(n) < 1:
                    return ans
                if len(n) > 4 or n[0] == '-':
                    parts[i] = False
                    break
                try:
                    value = int(n, 16)
                    parts[i] = (0 <= value < 1<<16)
                except:
                    parts[i] = False
                    break
            ans = 'IPv6' if all(parts) else ans
        return ans