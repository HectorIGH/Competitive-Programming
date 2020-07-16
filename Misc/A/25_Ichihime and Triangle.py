from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for _ in range(t):
        a, b, c, d = map(int, stdin.readline().split(' '))
        x, y = b, c

        for z in range(c, d + 1):
            if x + y >= z and x + z >= y and y + z >= x:
                ans.append(' '.join(map(str, [x, y, z])))
                break

    stdout.write('\n'.join(map(str, ans)))

if __name__ == "__main__":
    main()