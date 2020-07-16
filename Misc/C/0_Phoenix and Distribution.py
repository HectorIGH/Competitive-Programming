from sys import stdin, stdout

def main():

    t = int(stdin.readline())

    for _ in range(t):
        n, k = map(int, stdin.readline().split(' '))
        s = sorted(stdin.readline())[1:]

        res = []

        if s[0] != s[k - 1]:
            stdout.write(s[k - 1])
            stdout.write('\n')
            continue
        else:
            res.append(s[0])
            if s[k] != s[-1]:
                for i in range(k, n):
                    res.append(s[i])
            else:
                for _ in range((n - 1) // k):
                    res.append(s[-1])
        stdout.write(''.join(res))
        stdout.write('\n')


if __name__ == "__main__":
    main()