from sys import stdin, stdout

def main():

    t = int(stdin.readline())

    for _ in range(t):
        a = int(stdin.readline())

        pre = []
        mod = 10
        while a:
            re = a % mod
            if re:
                pre.append(str(re))
                a -= re
            mod *= 10

        stdout.write(str(len(pre))+'\n')
        stdout.write(' '.join(pre)+'\n')


if __name__ == "__main__":
    main()