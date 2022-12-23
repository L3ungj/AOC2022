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

use std::collections::HashMap;
use std::collections::HashSet;

fn any_in_neighbourhood(pos: &(i32, i32), hs: &HashSet<&(i32, i32)>) -> bool {
    let (x, y) = pos;
    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if hs.contains(&(x + dx, y + dy)) {
                return true;
            }
        }
    }
    return false;
}

fn dir_contain(pos: &(i32, i32), dir: &i32, hs: &HashSet<&(i32, i32)>) -> bool {
    // N S W E
    let (x, y) = *pos;
    match dir {
        0 => {
            hs.contains(&(x - 1, y)) || hs.contains(&(x - 1, y - 1)) || hs.contains(&(x - 1, y + 1))
        }
        1 => {
            hs.contains(&(x + 1, y)) || hs.contains(&(x + 1, y - 1)) || hs.contains(&(x + 1, y + 1))
        }
        2 => {
            hs.contains(&(x, y - 1)) || hs.contains(&(x - 1, y - 1)) || hs.contains(&(x + 1, y - 1))
        }
        3 => {
            hs.contains(&(x, y + 1)) || hs.contains(&(x - 1, y + 1)) || hs.contains(&(x + 1, y + 1))
        }
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let mut x = 0;
    let mut elves_pos: Vec<(i32, i32)> = Vec::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let mut y = 0;
        for c in s.chars() {
            if c == '#' {
                elves_pos.push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    let n = elves_pos.len();
    // N S W E
    for step in 0..100000000 {
        let elves_pos_hs: HashSet<_> = elves_pos.iter().collect();
        let mut new_elves_pos: Vec<(i32, i32)> = vec![(0, 0); n];
        for i in 0..n {
            new_elves_pos[i] = elves_pos[i];
        }
        for i in 0..n {
            if !any_in_neighbourhood(&elves_pos[i], &elves_pos_hs) {
                continue;
            }
            'dir_loop: for d in 0..4 {
                let dir = (step + d) % 4;
                if !dir_contain(&elves_pos[i], &dir, &elves_pos_hs) {
                    new_elves_pos[i] = match dir {
                        0 => (elves_pos[i].0 - 1, elves_pos[i].1),
                        1 => (elves_pos[i].0 + 1, elves_pos[i].1),
                        2 => (elves_pos[i].0, elves_pos[i].1 - 1),
                        3 => (elves_pos[i].0, elves_pos[i].1 + 1),
                        _ => panic!("Invalid direction"),
                    };
                    break 'dir_loop;
                }
            }
        }
        let mut new_elves_pos_cnt: HashMap<(i32, i32), i32> = HashMap::new();
        for i in 0..n {
            new_elves_pos_cnt
                .entry(new_elves_pos[i])
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        for i in 0..n {
            if new_elves_pos_cnt[&new_elves_pos[i]] > 1 {
                new_elves_pos[i] = elves_pos[i];
            }
        }
        // println!("{}", step+1);
        if elves_pos == new_elves_pos {
            println!("{}", step + 1);
            break;
        }
        elves_pos = new_elves_pos;
    }
}
