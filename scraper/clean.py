f = open('data/names.txt', 'r')
text = f.read()
f.close()

names = text.split('\n')
out = []

for name in names:
    i = name.index('[')
    year = name[i + 1 : i + 3]
    if year=='13':
        out.append(name)

outText = ''
for name in out:
    outText += name + '\n'

f = open('data/names.txt', 'w')
f.write(outText)
f.close()

