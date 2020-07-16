from sys import stdin, stdout

def main():

    t = int(stdin.readline())
    ans = []
    sizes = []

    for j in range(t):
        n, k = map(int, stdin.readline().split(' '))

        a = map(int, stdin.readline().split(' '))
        a = set(a)
        
        if len(a) > k:
            print(-1)
            continue
        else:
            a = list(a)
            for i in range(k-len(a)):
                a.append(1)
            print(len(a*n))
            print(*(a*n), sep = ' ')

if __name__ == "__main__":
    main()