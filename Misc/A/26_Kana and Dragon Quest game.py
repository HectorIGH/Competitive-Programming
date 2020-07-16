from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for _ in range(t):
        a, b, c = map(int, stdin.readline().split(' '))

        if a <= 20:
            for _ in range(c):
                a -= 10
        else:
            for _ in range(b):
                a = (a>>1) + 10
                if a <= 20:
                    break
            for _ in range(c):
                a -= 10

        ans.append('YES' if a <= 0 else 'NO')

    stdout.write('\n'.join(map(str, ans)))

if __name__ == "__main__":
    main()