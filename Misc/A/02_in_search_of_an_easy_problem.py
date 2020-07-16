n = int(input())

answers = input().split(' ')

answers = list(map(int, answers))

res = 'HARD' if 1 in answers else 'EASY'

print(res)