#!/usr/bin/env python

lines = []

with open('bin/webhogg-wasm.js', 'r') as f:
    for line in f:
        if 'const imports = {};' in line:
            lines.append('const imports = { js: { mem: init.memory } };\n')
        else:
            lines.append(line)

with open('bin/webhogg-wasm.js', 'w') as f:
    f.write(''.join(lines))
