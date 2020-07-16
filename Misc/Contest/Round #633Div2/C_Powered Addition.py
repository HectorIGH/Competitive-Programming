from math import log2, ceil, floor
t = int(input())

def find_index(a, n):
    diff = 0
    initial_index = n
    num = 0
    for i in range(1, n):
        diff = a[i] - a[i - 1]
        if diff < 0:
            initial_index = i
            num = a[i]
            break
    final_index = n
    for i in range(initial_index + 1, n):
        if a[i] > num:
            final_index = i
            break

    return diff, initial_index, final_index

ans = [0] * t

for test in range(t):
    n = int(input())
    a = list(map(int, input().split(' ')))

    diff, initial_index, final_index = find_index(a, n)

    #sec = int(ceil(log2(abs(diff)))) + 1
    sec = 0
    print('first', initial_index, 'final', final_index, 'sec', sec)
    while initial_index != final_index:
        sec = int(ceil(log2(abs(diff)))) + 1
        print('first', initial_index, 'final', final_index, 'sec', sec, 'diff', diff)
        for i in range(initial_index, final_index):
            a[i] += 2 ** (sec - 1)
        print(a)
        diff, initial_index, final_index = find_index(a, n)
        
        #break

    ans[test] = sec

print(*ans, sep='\n')