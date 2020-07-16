n, k = list(map(int, input().split(' ')))

res = 0

while n <= k:
    n *= 3
    k *= 2
    res += 1

print(res)