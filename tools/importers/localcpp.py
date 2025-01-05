import sys

lines = open(sys.argv[1]).readlines()
new_lines = []
stack = []
current = True
for line in lines:
    if line.strip().startswith("#ifdef"):
        pass
    elif line.strip().startswith("#if"):
        stack.append(current)
        continue
    elif line.strip().startswith("#elif") or line.strip().startswith("#else"):
        current = False
        continue
    elif line.strip().startswith("#endif"):
        if len(stack) > 0:
            current = stack.pop()
            continue
    if current:
        new_lines.append(line)
open(sys.argv[1], "w").write("".join(new_lines))

