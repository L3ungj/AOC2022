import sys
import os

if len(sys.argv) < 2:
    print("Usage: py create.py <name>")
    exit(0)

s = sys.argv[1]
if os.path.exists(f'src/{s}.rs'):
    print(f"File {s}.rs already exists")
    exit(0)

fi = open('template.rs', 'r')
if s.endswith('b'):
    try:
        fi = open(f'src/{s[:-1]}a.rs')
    except:
        pass

fo = open(f'src/{s}.rs', 'w')
fo.write(fi.read())

toml_fi = open('Cargo.toml', 'r')
toml_content = toml_fi.read()
toml_fi.close()
add_string = (
    "[[bin]]\n"
    f'name="day{s}"\n'
    f'path="src/{s}.rs"\n\n'
)
if f'day{s}' in toml_content:
    exit(0)
toml_content += add_string
with open('Cargo.toml', 'w') as toml_fo:
    toml_fo.write(toml_content)

