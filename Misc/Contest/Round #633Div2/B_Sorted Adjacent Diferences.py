t = int(input())

#ans = []

for _ in range(t):
    n = int(input())
    a = sorted(list(map(int, input().split(' '))), reverse = True)
    b = []

    for i in range(n//2):
        #print(a[i], a[-i - 1])
        b.append(a[-i-1])
        b.append(a[i])
    if n&1:
        b.append(a[n//2])
    
    print(*b[::-1])
    #ans.append(*b[::-1])

#print(*ans, sep = '\n')