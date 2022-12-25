fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim_end().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn rd_int(s: &String) -> i32 {
    let end = s.find(' ');
    if end == None {
        return s.parse().unwrap();
    }
    s[..end.unwrap()].parse().unwrap()
}

type Pt = (usize, usize);
use std::collections::HashSet;

fn get_min_steps(st: Pt, ed: Pt, obstacles: &mut Vec<(Pt, usize)>, n: &usize, m: &usize) -> usize {
    let mut q: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; *m]; *n]; 2];
    q[0][st.0][st.1] = true;
    for step in 0..1000000000 {
        let now = step % 2;
        let nxt = (step + 1) % 2;
        for x in 0..*n {
            for y in 0..*m {
                if q[now][x][y] {
                    if (x, y) == ed {
                        return step;
                    }
                }
            }
        }
        for obs in &mut *obstacles {
            let (x, y) = obs.0;
            let dir = obs.1;
            let mut nx = x;
            let mut ny = y;
            match dir {
                0 => nx -= 1,
                1 => ny += 1,
                2 => nx += 1,
                3 => ny -= 1,
                4 => continue,
                _ => (),
            }
            if nx == 0 {
                nx = n - 2;
            }
            if ny == 0 {
                ny = m - 2;
            }
            if nx == n - 1 {
                nx = 1;
            }
            if ny == m - 1 {
                ny = 1;
            }
            obs.0 = (nx, ny);
        }
        let obs_set = obstacles.iter().map(|x| x.0).collect::<HashSet<_>>();
        for x in 0..*n {
            for y in 0..*m {
                if q[now][x][y] {
                    if x > 0 && !obs_set.contains(&(x - 1, y)) {
                        q[nxt][x - 1][y] = true;
                    }
                    if y < m - 1 && !obs_set.contains(&(x, y + 1)) {
                        q[nxt][x][y + 1] = true;
                    }
                    if x < n - 1 && !obs_set.contains(&(x + 1, y)) {
                        q[nxt][x + 1][y] = true;
                    }
                    if y > 0 && !obs_set.contains(&(x, y - 1)) {
                        q[nxt][x][y - 1] = true;
                    }
                    if !obs_set.contains(&(x, y)) {
                        q[nxt][x][y] = true;
                    }
                    q[now][x][y] = false;
                }
            }
        }
    }
    0
}

fn main() {
    let mut obstacles: Vec<(Pt, usize)> = Vec::new();
    let mut x = 0;
    let mut m = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let mut y = 0;
        for c in s.chars() {
            match c {
                '^' => obstacles.push(((x, y), 0)),
                '>' => obstacles.push(((x, y), 1)),
                'v' => obstacles.push(((x, y), 2)),
                '<' => obstacles.push(((x, y), 3)),
                '#' => obstacles.push(((x, y), 4)),
                _ => (),
            }
            y += 1;
        }
        m = y;
        x += 1;
    }
    let n = x;
    let st = (0, 1);
    let ed = (n - 1, m - 2);
    let ans1 = get_min_steps(st, ed, &mut obstacles, &n, &m);
    let ans2 = get_min_steps(ed, st, &mut obstacles, &n, &m);
    let ans3 = get_min_steps(st, ed, &mut obstacles, &n, &m);
    println!("{}", ans1 + ans2 + ans3);
    // println!("{}", ans1);
    // println!("{}", ans2);
    // println!("{}", ans3);
}
