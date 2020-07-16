from collections import Counter

n = int(input())

p = input()

p = Counter(p)

if p['A'] > p['D']:
    print('Anton')
elif p['A'] < p['D']:
    print('Danik')
else:
    print('Friendship')
