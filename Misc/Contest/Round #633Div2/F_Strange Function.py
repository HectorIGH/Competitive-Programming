n = int(input())

a = map(int, input().split(' '))

p = map(int, input().split(' '))

m = int(input())

b = list(map(int, input().split(' ')))

coins = 0

look = b[-1]
start = 1
for i in range(1, n):
    if a[-i] > look:
        coins += p[-i]
    if a[-i] <= look and p[-i] < 0:
        coins += p[-i]
    if a[-i] == look:
        start = max(start, i)



for i in range(1, m):
    look = b[-i]
    nxt = b[-i - 1]
    for i in range(1, n + 1):
        if a[-i] < look and p[-i] < 0:
            coins += p[-i]