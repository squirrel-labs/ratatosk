#!/usr/bin/env python

exit()

lines = []

with open('bin/webhogg-wasm.js', 'r') as f:
    for line in f:
        if 'const imports = {};' in line:
            lines.append('const imports = { js: { mem: init.memory } };console.info("oho", imports);\n')
            #lines.append('const imports = {};\n')
        else:
            lines.append(line)

with open('bin/webhogg-wasm.js', 'w') as f:
    f.write(''.join(lines))
