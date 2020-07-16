t = int(input())

for _ in range(t):
    n = int(input())
    current_plays, current_clears = 0, 0
    possible = True
    for i in range(n):
        plays, clears = map(int, input().split(' '))
        dp = plays - current_plays
        dc = clears - current_clears
        if dp >= 0 and dc >= 0:
            if dp >= dc:
                current_clears = clears
                current_plays = plays
                continue
            else:
                possible = False
                continue
        else:
            possible = False
    
    print('YES' if possible else 'NO')
            