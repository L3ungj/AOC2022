import subprocess
import sys

if len(sys.argv) < 2:
    print("Usage: py create.py <name>")
    exit(0)

s = sys.argv[1]
proc = subprocess.run([
    'rustc', f'./src/{s}.rs',
    '-A', 'unused_variables',
    '-A', 'dead_code',
    '-A', 'non_camel_case_types',
    '-o', f'./target/debug/{s}.exe'
])
if proc.returncode != 0:
    exit(0)

fi = open('fi.txt', 'r')
input_string = fi.read() + '\n@#-3aV[./'

fo = open('fo.txt', 'w')
proc = subprocess.run([
    f'./target/debug/{s}.exe',
],input=input_string.encode('utf-8'), stdout=fo)
