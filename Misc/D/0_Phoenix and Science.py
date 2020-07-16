from sys import stdin, stdout

def main():

    t = int(stdin.readline())

    for _ in range(t):
        n = int(stdin.readline())
        ans = []

        i = 1
        while i <= n:
            ans.append(i)
            n -= i
            i *= 2
        #print(ans, n)
        if n > 0:
            ans.append(n)
            ans.sort()
        print(len(ans) - 1)
        #print(ans)
        for i in range(1, len(ans)):
            print(ans[i] - ans[i - 1], end = ' ')


if __name__ == "__main__":
    main()