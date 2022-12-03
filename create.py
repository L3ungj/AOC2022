fi = open('template.rs', 'r')

s = input()
fo = open(f'src/{s}.rs', 'w')

fo.write(fi.read())
