// add \n at end of input

use std::collections::VecDeque;

type Monkey = (String, i32, usize, usize);
fn apply(operation: &String, old: Vec<i32>, mods: &Vec<i32>) -> Vec<i32> {
    let tokens: Vec<String> = operation.split(' ').map(String::from).collect();
    let n = old.len();
    let lval;
    let rval;
    let op = &tokens[1];
    let mut v_t_l : Vec<i32> = Vec::new();
    let mut v_t_r : Vec<i32> = Vec::new();
    if tokens[0] == "old" {
        lval = &old;
    } else {
        let t = tokens[0].parse().unwrap();
        v_t_l.resize(n, t);
        lval = &v_t_l;
    }
    if tokens[2] == "old" {
        rval = &old;
    } else {
        let t = tokens[2].parse().unwrap();
        v_t_r.resize(n, t);
        rval = &v_t_r;
    }
    if *op == "+" {
        return my_add(lval, rval, mods);
    } else if *op == "*" {
        return my_mul(lval, rval, mods);
    }
    return Vec::new();
}

fn my_add(a: &Vec<i32>, b:&Vec<i32>, mods : &Vec<i32>) -> Vec<i32>{
    let n = a.len();
    let mut c : Vec<i32> = Vec::new();
    c.resize(n, 0);
    for i in 0..n {
        c[i] = (a[i] + b[i]) %mods[i];
    }
    return c;
}
fn my_mul(a: &Vec<i32>, b:&Vec<i32>, mods : &Vec<i32>) -> Vec<i32>{
    let n = a.len();
    let mut c : Vec<i32> = Vec::new();
    c.resize(n, 0);
    for i in 0..n {
        c[i] = a[i] * b[i] %mods[i];
    }
    return c;
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut v: Vec<Vec<i32>> = Vec::new();
    let mut v_mod: Vec<VecDeque<Vec<i32>>> = Vec::new();
    let mut mods : Vec<i32> = Vec::new();
    'main: loop {
        let mut ss: Vec<String> = Vec::new();
        for _ in 0..7 {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();
            if s == "@#-3aV[./" {
                break 'main;
            }
            ss.push(s);
        }
        monkeys.push((
            ss[2][17..].to_string(),
            ss[3][19..].parse().unwrap(),
            ss[4][25..].parse().unwrap(),
            ss[5][26..].parse().unwrap(),
        ));
        let mut items = Vec::<i32>::new();
        let tokens: Vec<String> = ss[1][16..].split(',').map(String::from).collect();
        for token in tokens {
            items.push(token.trim().parse().unwrap());
        }
        v.push(items);
    }
    let n = monkeys.len();
    let mut cnt: Vec<i32> = Vec::new();
    cnt.resize(n, 0);
    for i in 0..n {
        mods.push(monkeys[i].1);
    }
    v_mod.resize(n, VecDeque::new());
    for i in 0..n {
        for a in &v[i] {
            let mut v_t: Vec<i32> = Vec::new();
            for j in 0..n {
                v_t.push(a % mods[j]);
            }
            v_mod[i].push_back(v_t);
        }
    }
    for _ in 0..10000 {
        for i in 0..n {
            while !v_mod[i].is_empty() {
                let cur =(*v_mod[i].front().unwrap()).to_vec();
                v_mod[i].pop_front();
                let new = apply(&monkeys[i].0, cur, &mods);
                cnt[i] += 1;
                if new[i] == 0 {
                    v_mod[monkeys[i].2].push_back(new);
                }
                else {
                    v_mod[monkeys[i].3].push_back(new);
                }
            }
        }
    }
    cnt.sort();
    cnt.reverse();
    println!("{}", cnt[0] as i64 * cnt[1] as i64);
}
