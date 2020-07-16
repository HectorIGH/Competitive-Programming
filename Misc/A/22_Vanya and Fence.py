n, h = map(int, input().split())

p = list(map(int, input().split()))

#ans = 0
#
#for e in p:
#    if e <= h:
#        ans += 1
#    else:
#        ans += 2
#
#print(ans)

ans = sum(map(lambda x : 1 if x <= h else 2, p))

print(ans)