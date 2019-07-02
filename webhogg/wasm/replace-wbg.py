f = open('bin/webhogg-wasm.wasm', 'rb')
b = f.read()
f.close()

f = open('bin/webhogg-wasm.wasm', 'wb')
f.write(b.replace(b'env', b'wbg'))
f.close()
