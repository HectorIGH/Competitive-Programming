#from math import gcd

n, m = map(int, input().split(' '))
s = input().split(' ')
t = input().split(' ')
times = int(input())

#minimo = min(n, m)
#maximo = max(n, m)
#mcd = gcd(n, m)

#tamano = int(minimo / mcd * maximo)

#s_mult = int(n * (m / mcd))
#t_mult = int(m * (n / mcd))

#names = list(zip(s * s_mult, t * t_mult))

for _ in range(times):
    #year = int(input()) % tamano - 1
    year = int(input()) - 1
    #print(''.join(names[year]))
    print(''.join((s[year % n], t[year % m])))