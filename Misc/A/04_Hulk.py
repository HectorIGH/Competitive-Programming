n = int(input())

ans = 'I hate'

for i in range(1, n):
    ans = ' '.join([ans, 'that I love']) if i & 1 else ' '.join([ans, 'that I hate'])

ans = ' '.join([ans, 'it'])

print(ans)