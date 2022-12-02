import subprocess

s = input()
proc = subprocess.run([
    'rustc', f'./src/{s}.rs',
    '-o', f'./src/{s}.exe'
])
if proc.returncode != 0:
    exit(0)

fi = open('fi.txt', 'r')
fo = open('fo.txt', 'w')
subprocess.run([
    f'./src/{s}.exe',
],stdin=fi, stdout=fo)