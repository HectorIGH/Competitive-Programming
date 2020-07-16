n = int(input())

steps = [5, 4, 3, 2, 1]
total = 0

for s in steps:
    total += n // s
    n = n % s
    if not n:
        break

print(total)
