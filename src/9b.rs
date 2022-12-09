use std::collections::HashSet;

fn is_adj(a: (i32, i32), b: (i32, i32)) -> bool {
    if a.0 + 1 == b.0 && a.1 == b.1 {
        return true;
    }
    if a.0 - 1 == b.0 && a.1 == b.1 {
        return true;
    }
    if a.0 == b.0 && a.1 + 1 == b.1 {
        return true;
    }
    if a.0 == b.0 && a.1 - 1 == b.1 {
        return true;
    }
    return false;
}

fn is_in_neighbourhood(a: (i32, i32), b: (i32, i32)) -> bool {
    for di in -1..2 {
        for dj in -1..2 {
            let new_pos = (a.0 + di, a.1 + dj);
            if b == new_pos {
                return true;
            }
        }
    }
    return false;
}
fn main() {
    let mut snake: Vec<(i32, i32)> = Vec::new();
    snake.resize(10, (0, 0));
    let mut set = HashSet::<(i32, i32)>::new();
    set.insert((0, 0));
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./" {
            break;
        }
        let tokens: Vec<String> = s.split(' ').map(String::from).collect();
        let steps: i32 = tokens[1].parse().unwrap();
        for _ in 0..steps {
            match &*tokens[0] {
                "L" => snake[0].1 -= 1,
                "R" => snake[0].1 += 1,
                "U" => snake[0].0 -= 1,
                "D" => snake[0].0 += 1,
                _ => (),
            };
            // println!("{} {}", snake[0].0.to_string(), snake[0].1.to_string());
            for i in 0..9 {
                if !is_in_neighbourhood(snake[i], snake[i + 1]) {
                    let mut ok = false;
                    'out1: for di in -1..2 {
                        for dj in -1..2 {
                            let new_pos = (snake[i + 1].0 + di, snake[i + 1].1 + dj);
                            if is_adj(new_pos, snake[i]) {
                                snake[i + 1] = new_pos;
                                ok = true;
                                break 'out1;
                            }
                        }
                    }
                    if !ok {
                        'out1: for di in -1..2 {
                            for dj in -1..2 {
                                let new_pos = (snake[i + 1].0 + di, snake[i + 1].1 + dj);
                                if is_in_neighbourhood(new_pos, snake[i]) {
                                    snake[i + 1] = new_pos;
                                    break 'out1;
                                }
                            }
                        }
                    }
                }
                // println!(
                //     "{} {} {}",
                //     i.to_string(),
                //     snake[i + 1].0.to_string(),
                //     snake[i + 1].1.to_string()
                // );
            }
            set.insert(snake[9]);
        }
    }
    println!("{}", (set.len()).to_string());
}
