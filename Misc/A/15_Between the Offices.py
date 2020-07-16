n = int(input())

f = input()

#sf = 0
#fs = 0

#for i in range(1, n):
#    if f[i] == 'S' and f[i - 1] == 'F':
#        fs += 1
#    if f[i] == 'F' and f[i - 1] == 'S':
#        sf += 1

#print('YES' if sf > fs else 'NO')

print('YES' if f[0] == 'S' and f[-1] == 'F' else 'NO')