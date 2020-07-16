t = int(input())

for _ in range(t):
    n = int(input())
    health = []
    damage = []
    for i in range(n):
        a, b = map(int, input().split(' '))
        health.append(a)
        damage.append(b)
    bullets = 0
    first = True    
    while health:
        if first:
            target = health.index(min(health))
            first = False
        else:
            target = (target + 1 ) % len(health)
        print(target)

        if health[target] > 0:
            bullets += health[target]

        B = damage[target]
        #print('inflected damage: ', B)
        health[(target + 1) % len(health)] -= B

        health.pop(target)
        damage.pop(target)
        #print(bullets, list(zip(health, damage)))


    print(bullets)
            