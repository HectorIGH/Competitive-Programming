n = 5
row = 0
for i in range(n):
    str = list(map(int, input().split(' ')))
    if 1 in str:
        row = i
        col = str.index(1)

print(abs(2 - row) + abs(2 - col))