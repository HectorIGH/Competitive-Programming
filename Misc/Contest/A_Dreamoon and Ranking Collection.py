from math import log2

cases = int(input())

v = [0 for _ in range(cases)]

for case in range(cases):
    n, x = list(map(int, input().split(' ')))
    ranks = list(map(int, input().split(' ')))

    current_max = max(ranks)

    for i in range(1, x + 1):
        for place in range(i, current_max + 1 + x):
            if place not in ranks:
                v[case] = place
                ranks += [place]
                break
    
    ranks = sorted(list(set(ranks)))
    for r in ranks:
        if r > v[case] and r - v[case] == 1:
            v[case] = r


for e in v:
    print(e)

