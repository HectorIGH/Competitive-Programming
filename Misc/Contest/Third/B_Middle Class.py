t = int(input())

for _ in range(t):
    n, x = map(int, input().split(' '))
    a = sorted(map(int, input().split(' ')), reverse = True)

    #ans = sum(map(lambda s : 1 if s >= x else 0, a))

    index = 0
    while index < n:
        if a[index] >= x:
            index += 1
        else:
            break
    #print(index)
    base = sum(a[:index])

    ans = index

    suma = base
    #suma fo m people >= x * m

    for i in range(index, n):
        suma += a[i]
        if suma >= x * (i + 1):
            #print(suma, i)
            ans = max(ans, i + 1)
        else:
            break

    print(ans)
            