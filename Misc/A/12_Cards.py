from collections import Counter

n = int(input())

s = [l for l in input()]

l = Counter(s)

for _ in range(l['n']):
    print(1, end = ' ')
for _ in range(l['z']):
    print(0, end = ' ')