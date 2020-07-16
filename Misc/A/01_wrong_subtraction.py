n, k = input().split(' ')
n = int(n)
k = int(k)
for _ in range(k):
    n = n - 1 if n % 10 else n // 10

print(n)