strs = input()

size = len(strs)

a = sum([1 for e in strs if e == 'a'])

if a > size // 2:
    print(size)
else:
    print(2 * a - 1)