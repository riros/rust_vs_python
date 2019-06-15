from itertools import product
from operator import mul

from functools import reduce

# K, M = (int(x) for x in input().split())
# Ni = []
# for i in range(K):
#     Ni.append([int(x) for x in input().split()])
#     print(Ni)
#     del Ni[i][0]

from random import randint

K, M = 3, 1000
# Ni = [[2, 5, 6], [3, 7, 8, 9], [5, 5, 7, 8, 9, 10]]

Ni = []
print('load')
for i in range(K):
    print(i)
    Ni.append([randint(1, 7) for x in range(10 ** K)])
print('calc')
print(reduce((lambda x, y: x if x >= y else y), map(lambda x: sum(map(mul, x, x)) % M, product(*Ni))))
