n = int(input())
'''
winner = 'Ehab'
 
while n:
    if n > 1:
        n -= n - 1 if n & 1 else n
        winner = 'Mahmoud'
    if n > 0:
        n -= n if n & 1 else n
        winner = 'Ehab'
 
print(winner)
'''
if n & 1:
    print('Ehab')
else:
    print('Mahmoud')