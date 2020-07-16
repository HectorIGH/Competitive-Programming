n = int(input())

for i in range(n):
    str = input()
    if str[-1] == 'o':
        print('FILIPINO')
    elif str[-1] == 'u':
        print('JAPANESE')
    else:
        print('KOREAN')