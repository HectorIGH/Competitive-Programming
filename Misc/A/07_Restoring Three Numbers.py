n = list(map(int, input().split(' ')))

n = sorted(n)

print(n[-1] - n[0], n[-1] - n[1], n[-1] - n[2])