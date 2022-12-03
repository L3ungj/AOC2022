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
proc = subprocess.Popen([
    f'./src/{s}.exe',
],stdin=subprocess.PIPE, stdout=fo)

proc.communicate(input_string.encode('utf-8'))