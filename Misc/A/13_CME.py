def foo(n):
    if n & 1: # Odd
        #sum = n // 2 + 1 #n // 2 + n % 2
        # If n is odd n % 2 is always 1
        # n//2 is equivalent to (n - 1) / 2
        # If we add +1 adn then double it will be grater than n by 1
        # If we double without adding +1 adn then double it will be lower than n by 1
        #return 2 * sum - n
        return 1
    if n == 2:
        return 2
    else:
        return 0

times = int(input())

for _ in range(times):
    n = int(input())
    print(foo(n))