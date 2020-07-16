from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for _ in range(t):
        n, k = map(int, stdin.readline().split(' '))

        if n == k:
            ans.append(n + 1)
            continue
        if k < n:
            ans.append(k)
            continue
        else:
            block = k // (n - 1)
            block += 0 if k == (n - 1) * block else 1
            before = (n - 1) * (block - 1)
            leftover = k - before
            ans.append((block - 1) * n + leftover)

    stdout.write('\n'.join(map(str, ans)))

if __name__ == "__main__":
    main()