use std::mem::swap;

type ll = i64;
type vi = Vec<ll>;

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn draw_line(mp: &mut Vec<Vec<bool>>, mut a: usize, mut b: usize, mut c: usize, mut d: usize) {
    if a == c {
        if b > d {
            swap(&mut b, &mut d);
        }
        for j in b..(d + 1) {
            // println!("drawn {} {}", a, j);
            mp[a][j] = true;
        }
    } else if b == d {
        if a > c {
            swap(&mut a, &mut c);
        }
        for i in a..(c + 1) {
            // println!("drawn {} {}", i, b);
            mp[i][b] = true;
        }
    }
}

fn drop_sand(mp: &mut Vec<Vec<bool>>) -> bool {
    let mut x = 500;
    let mut y = 1;
    while y < 200 {
        // println!("{} {}", x, y);
        if !mp[x][y] {
        } else if !mp[x - 1][y] {
            x -= 1;
        } else if !mp[x + 1][y] {
            x += 1;
        } else {
            mp[x][y - 1] = true;
            return true;
        }
        y += 1;
    }
    false
}

fn main() {
    let mut mp: Vec<Vec<bool>> = vec![vec![false; 200]; 1000];
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let v: Vec<String> = s.split(" -> ").map(String::from).collect();
        let n = v.len();
        for i in 1..n {
            let co1: Vec<String> = v[i - 1].split(",").map(String::from).collect();
            let a = co1[0].parse().unwrap();
            let b = co1[1].parse().unwrap();
            let co2: Vec<String> = v[i].split(",").map(String::from).collect();
            let c = co2[0].parse().unwrap();
            let d = co2[1].parse().unwrap();
            // println!("{} {} {} {}", a, b, c, d);
            draw_line(&mut mp, a, b, c, d);
        }
    }
    let mut cnt = 0;
    while drop_sand(&mut mp) {
        cnt += 1;
    }
    println!("{}", cnt);
}
