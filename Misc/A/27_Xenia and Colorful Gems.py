from sys import stdin, stdout
from bisect import bisect_left, bisect_right

def aux(red, green, blue, w):
    for x in red:
        y = bisect_left(green, x)
        z = bisect_right(blue, x)
        if y == len(green) or z == 0:
            continue
        z -= 1
        value = (x - green[y])*(x - green[y]) + (green[y] - blue[z])*(green[y] - blue[z]) + (blue[z] - x)*(blue[z] - x)
        w = min(w, value)
    return w

def main():

    t = int(stdin.readline())

    ans = []
    
    for _ in range(t):
        r, g, b = map(int, stdin.readline().split(' '))
        red = list(sorted(map(int, stdin.readline().split(' '))))
        green = list(sorted(map(int, stdin.readline().split(' '))))
        blue = list(sorted(map(int, stdin.readline().split(' '))))

        w = float('inf')
        w = aux(red, green, blue, w)
        w = aux(red, blue, green, w)
        w = aux(green, blue, red, w)
        w = aux(green, red, blue, w)
        w = aux(blue, green, red, w)
        w = aux(blue, red, green, w)
        
        ans.append(w)

    stdout.write('\n'.join(map(str, ans)))

if __name__ == "__main__":
    main()