n = int(input())

ways = 0
for i in range(1, n):
    if ((n - i) // i) * i == (n - i):
        ways += 1

print(ways)