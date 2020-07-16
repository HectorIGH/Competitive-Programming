# TIMEOUT
# Possibly using two indexes, one from beggining and one in tail to track position

n = int(input())

s = sorted(map(int, input().split(' ')), reverse = True)

suma = 0

while s:
    if s[0] == 4:
        if s[-1] == 4:
            suma += len(s)
            break
        suma += 1
        s.pop(0)
        continue
    if s[0] == 3:
        if s[-1] == 1: # group with a 1
            s.pop(0)
            s.pop(-1)
        else: # there is no 1 left
            if s[-1] == 3: # Checking for only 3 remaining
                suma += len(s)
                break
            s.pop(0)
        suma += 1
        continue
    if s[0] == 2:
        if s[-1] == 1: # There is a 1
            if s[-2] == 1: # Check if there is another 1
                s.pop(-2)
            s.pop(-1)
        elif s[-1] == 2 and len(s) > 1:
            #print(suma)
            suma += (len(s) // 2) + (len(s) & 1)
            #print(suma, len(s))
            break
        s.pop(0)
        suma += 1
        continue
    if s[0] == 1:
        suma += sum(s) // 4 + sum(s) % 4 if len(s) > 4 else 1
        break

print(suma)