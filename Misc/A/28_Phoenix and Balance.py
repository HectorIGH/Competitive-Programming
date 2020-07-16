from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for _ in range(t):
        n = int(stdin.readline())

        coins = [1<<i for i in range(1, n + 1)]
        
        res = 0
        for i in range(n//2):
            res += coins[i + 1] - coins[i]
        

        ans.append(res)

    stdout.write('\n'.join(map(str, ans)))

if __name__ == "__main__":
    main()