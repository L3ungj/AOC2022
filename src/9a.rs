use std::collections::HashSet;

fn is_adj(a : (i32, i32), b: (i32, i32)) -> bool {
    if a.0 + 1 == b.0 && a.1 == b.1 {return true;}
    if a.0 - 1 == b.0 && a.1 == b.1 {return true;}
    if a.0 == b.0 && a.1 + 1 == b.1 {return true;}
    if a.0 == b.0 && a.1 - 1 == b.1 {return true;}
    return false;
}

fn main()
{
    let mut head : (i32, i32) = (0, 0);
    let mut tail : (i32, i32) = (0, 0);
    let mut set = HashSet::<(i32, i32)>::new();
    set.insert(tail);
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./"
        {
            break;
        }
        let tokens : Vec<String> = s.split(' ').map(String::from).collect();
        let steps:i32 = tokens[1].parse().unwrap();
        for _ in 0..steps {
            match &*tokens[0] {
                "L"=> head.1 -= 1,
                "R" => head.1 += 1,
                "U" => head.0 -= 1,
                "D" => head.0 += 1,
                _=> (),
            };
            let mut in_neighbourhood = false;
            for di in -1..2 {
                for dj in -1..2 {
                    let new_pos = (head.0 + di, head.1 + dj);
                    if tail == new_pos {
                        in_neighbourhood = true;
                    }
                }
            }
            if !in_neighbourhood {
                'out1 : for di in -1..2 {
                    for dj in -1..2 {
                        let new_pos = (tail.0 + di, tail.1 + dj);
                        if is_adj(new_pos, head) {
                            tail = new_pos;
                            set.insert(tail);
                            break 'out1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", (set.len()).to_string());
}