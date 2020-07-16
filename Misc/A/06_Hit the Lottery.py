n = int(input())

bills = [100, 20, 10, 5, 1]

amount = 0

for bill in bills:
    if bill <= n:
        amount += n // bill
        n = n % bill

print(amount)
