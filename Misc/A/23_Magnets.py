n = int(input())

groups = 0

current = 0

p = [0] * n

for i in range(n):
    p[i] = int(input())

for prox in p:
    if current != prox:
        groups += 1
        current = prox

print(groups)