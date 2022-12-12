type ll = i64;
type vi = Vec<ll>;
type pii = (usize, usize);

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn in_bounds(pos : pii, n: &usize, m: &usize) -> bool{
    return pos.0 < *n && pos.1 < *m;
}
fn main() {
    let mut mp : Vec<vi> = Vec::new();
    let mut start : pii = (0, 0);
    let mut end: pii = (0, 0);
    let mut i = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let mut v_t : vi = Vec::new();
        let mut j = 0;
        for c in s.bytes() {
            if c == 'S' as u8 {
                start = (i, j);
                v_t.push(1);
            }
            else if c == 'E' as u8 {
                end = (i, j);
                v_t.push(26);
            }
            else {
                v_t.push(c as i64 - 'a' as i64 + 1);
            }
            j += 1;
        }
        i += 1;
        mp.push(v_t);
    }
    let n = mp.len();
    let m = mp[0].len();
    let mut q : Vec<Vec<Vec<bool>>> = vec![vec![vec![false;m];n];2];
    let dirs: Vec<pii> = vec![(1, 0), (0, 1)];
    q[0][start.0][start.1] = true;
    'bye: for step in 0..100000000 {
        println!("{}", step);
        let now = step % 2;
        let nxt = (step+1) % 2;
        for i in 0..n {
            for j in 0..m {
                if q[now][i][j] {
                    let cur = (i, j);
                    q[now][i][j] = false;
                    if cur == end {
                        println!("{}", step);
                        break 'bye;
                    }
                    for d in &dirs {
                        let new = (cur.0 + d.0, cur.1 + d.1);
                        if !in_bounds(new, &n, &m) {
                            continue;
                        }
                        if mp[new.0][new.1] <= mp[cur.0][cur.1] + 1 {
                            q[nxt][new.0][new.1] = true;
                        }
                    }
                    for d in &dirs {
                        if cur.0 < d.0 || cur.1 < d.1 {
                            continue;
                        }
                        let new = (cur.0 - d.0, cur.1 - d.1);
                        if !in_bounds(new, &n, &m) {
                            continue;
                        }
                        if mp[new.0][new.1] <= mp[cur.0][cur.1] + 1 {
                            q[nxt][new.0][new.1] = true;
                        }
                    }

                }
            }
        }
    }

}
