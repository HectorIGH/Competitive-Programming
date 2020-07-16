table = input()

hand = input().split(' ')

rank = list(map(lambda x : x[0], hand))
suit = list(map(lambda x : x[1], hand))


print('YES' if table[0] in rank or table[1] in suit else 'NO')