f = open('../api/data/names.txt', 'r')
names = f.read().split('\n')
codes = []
for n in names:
    code = n.split('#')[1][1:]
    codes.append(code)

import os

for f in os.listdir('../api/data'):
    if (f != 'names.txt'):
        if f[:-4] not in codes:
            os.remove('../api/data/'+f)

print(codes)