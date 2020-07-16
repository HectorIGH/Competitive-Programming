from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []

    for j in range(t):
        a, b = sorted(map(int, stdin.readline().rstrip().split(' ')))
        
        ans.append(str(a+b))


    stdout.write('\n'.join(ans))


if __name__ == "__main__":
    main()