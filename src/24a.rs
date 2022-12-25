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
    let mut q: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; m]; n]; 2];
    q[0][st.0][st.1] = true;
    for step in 0..1000000000 {
        // for obs_pos in &obstacles {
        //     println!("obs0 {} {} at {}", obs_pos.0.0, obs_pos.0.1, step);
        // }
        let mut new_obstacles: Vec<(Pt, usize)> = Vec::new();
        for obs in &obstacles {
            let (x, y) = obs.0;
            let dir = obs.1;
            let mut nx = x;
            let mut ny = y;
            match dir {
                0 => nx -= 1,
                1 => ny += 1,
                2 => nx += 1,
                3 => ny -= 1,
                4 => {
                    new_obstacles.push(((nx, ny), dir));
                    continue;
                }
                _ => (),
            }
            if nx == 0 {
                nx = n-2;
            }
            if ny == 0 {
                ny = m-2;
            }
            if nx == n-1 {
                nx = 1;
            }
            if ny == m-1 {
                ny = 1;
            }
            new_obstacles.push(((nx, ny), dir));
        }
        obstacles = new_obstacles;
        // for obs_pos in &obstacles {
        //     println!("obs1 {} {} at {}", obs_pos.0.0, obs_pos.0.1, step);
        // }
        let obs_set: HashSet<Pt> = obstacles.iter().map(|x| x.0).collect();
        // for obs_pos in &obs_set {
        //     println!("obs2 {} {} at {}", obs_pos.0, obs_pos.1, step);
        // }
        let now = step % 2;
        let nxt = (step + 1) % 2;
        for i in 0..n {
            for j in 0..m {
                if q[now][i][j] {
                    // println!("{} {} {}", i, j, step);
                    if (i, j) == ed {
                        println!("{}", step);
                        return;
                    }
                    for dir in [(-1, 0), (1, 0), (0, 1), (0, -1), (0, 0)] {
                        let nx = i as i32 + dir.0;
                        let ny = j as i32 + dir.1;
                        if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 {
                            continue;
                        }
                        let nx = nx as usize;
                        let ny = ny as usize;
                        // println!("new {} {} {}", nx, ny, step);
                        if obs_set.contains(&(nx, ny)) {
                            continue;
                        }
                        q[nxt][nx][ny] = true;
                    }
                    q[now][i][j] = false;
                }
            }
        }
    }
}
