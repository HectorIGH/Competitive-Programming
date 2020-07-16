from functools import reduce

n = int(input())

elements = [input() for _ in range(n)]

poly = {'Tetrahedron' : 4, 'Cube' : 6, 'Octahedron' : 8, 'Dodecahedron' : 12, 'Icosahedron' : 20}

print(sum([poly[i] for i in elements]))