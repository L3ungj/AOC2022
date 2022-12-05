import subprocess

s = input()
proc = subprocess.run([
    'rustc', f'./src/{s}.rs',
    '-o', f'./src/{s}.exe'
])
if proc.returncode != 0:
    exit(0)

fi = open('fi.txt', 'r')
input_string = fi.read() + '\n@#-3aV[./'

fo = open('fo.txt', 'w')
proc = subprocess.run([
    f'./src/{s}.exe',
],input=input_string.encode('utf-8'), stdout=fo)
