// add \n at end of input

use std::collections::VecDeque;

type Monkey = (String, i32, u32, u32);

fn apply(operation: &String, old: i32) -> i32 {
    let tokens: Vec<String> = operation.split(' ').map(String::from).collect();
    let lval;
    let rval;
    let op = &tokens[1];
    if tokens[0] == "old" {
        lval = old;
    } else {
        lval = tokens[0].parse().unwrap();
    }
    if tokens[2] == "old" {
        rval = old;
    } else {
        rval = tokens[0].parse().unwrap();
    }
    if *op == "+" {
        return lval + rval;
    } else if *op == "*" {
        return lval * rval;
    }
    return -1;
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut v: Vec<VecDeque<i32>> = Vec::new();
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
        let mut items = VecDeque::<i32>::new();
        let tokens: Vec<String> = ss[1][16..].split(',').map(String::from).collect();
        for token in tokens {
            items.push_back(token.trim().parse().unwrap());
        }
        v.push(items);
    }
    let n = monkeys.len();
    let mut cnt: Vec<i32> = Vec::new();
    cnt.resize(n, 0);
    for _ in 0..20 {
        for i in 0..n {
            while !v[i].is_empty() {
                let cur = *v[i].front().unwrap();
                let mut new = apply(&monkeys[i].0, cur);
                new = new / 3;
                cnt[i] += 1;
                if new % monkeys[i].1 == 0 {
                    v[monkeys[i].2].push_back(new);
                }
            }
        }
    }
}
