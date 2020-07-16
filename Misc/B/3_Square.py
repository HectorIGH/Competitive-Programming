from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for j in range(t):
        x1, y1 = sorted(map(int, stdin.readline().rstrip().split(' ')))
        x2, y2 = sorted(map(int, stdin.readline().rstrip().split(' ')))

        if y1 + y2 == x1 and x1 == x2:
            ans.append('Yes')
            continue
        if y1 + x2 == x1 and x1 == y2:
            ans.append('Yes')
            continue
        if x1 + y2 == y1 and y1 == x2:
            ans.append('Yes')
            continue
        if x1 + x2 == y1 and y1 == y2:
            ans.append('Yes')
            continue
        ans.append('No')


    stdout.write('\n'.join(ans))


if __name__ == "__main__":
    main()