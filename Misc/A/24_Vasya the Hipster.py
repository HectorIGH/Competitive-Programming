a, b = map(int, input().split(' '))

mini = min(a, b)

maxi = max(a, b)

print(mini, (maxi - mini) // 2)
