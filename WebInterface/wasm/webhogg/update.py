#!/usr/bin/env python

ISC = '//!IMPORTANT_STUFF'

def rf(fn):
    f = open(fn)
    c = f.read()
    f.close()
    return c

nc = rf('pkg/webhogg.js')
m1 = rf('loader.js')
m2 = rf('graphics.js')

nc = nc.split('function init(module) {')[0].strip('\n')
nc = nc.replace('export function ', 'function ')
m1 = m1.split('//!IMPORTANT_STUFF')[-1]
m2 = m2.split('//!IMPORTANT_STUFF')[-1]

nc += '\n' * 2 + ISC

m1 = nc + m1
m2 = nc + m2

with open('loader.js', 'w') as f:
    f.write(m1)

with open('graphics.js', 'w') as f:
    f.write(m2)
