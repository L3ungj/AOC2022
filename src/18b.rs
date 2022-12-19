type Vec3 = (i32, i32, i32);

use std::collections::HashSet;

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn add_vec3(a: Vec3, b: Vec3) -> Vec3 {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

// stack overflowed, wtf ?
// fn dfs(pos: &Vec3, hs: &HashSet<Vec3>, dirs: &Vec<Vec3>, exts: &mut HashSet<Vec3>) {
//     println!("{} {} {}", pos.0, pos.1, pos.2);
//     exts.insert(*pos);
//     for d in dirs {
//         let n_pos = add_vec3(*pos, *d);
//         if !hs.contains(&n_pos)
//             && !exts.contains(&n_pos)
//             && n_pos.0.abs() <= 23
//             && n_pos.1.abs() <= 23
//             && n_pos.2.abs() <= 23
//         {
//             dfs(&n_pos, hs, dirs, exts);
//         }
//     }
// }

fn bfs(pos: &Vec3, hs: &HashSet<Vec3>, dirs: &Vec<Vec3>, exts: &mut HashSet<Vec3>) {
    let mut q: Vec<Vec3> = Vec::new();
    q.push(*pos);
    while !q.is_empty() {
        let pos = q.pop().unwrap();
        if exts.contains(&pos) {
            continue;
        }
        exts.insert(pos);
        for d in dirs {
            let n_pos = add_vec3(pos, *d);
            if !hs.contains(&n_pos)
                && !exts.contains(&n_pos)
                && n_pos.0.abs() <= 23
                && n_pos.1.abs() <= 23
                && n_pos.2.abs() <= 23
            {
                q.push(n_pos);
            }
        }
    }
}

fn main() {
    let mut hs: HashSet<Vec3> = HashSet::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let raw_pt: Vec<i32> = s.split(',').map(|x| x.parse().unwrap()).collect();
        let pt = (raw_pt[0], raw_pt[1], raw_pt[2]);
        hs.insert(pt);
    }
    let dirs: Vec<Vec3> = vec![
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut exts: HashSet<Vec3> = HashSet::new();
    bfs(&(-23, -23, -23), &hs, &dirs, &mut exts);
    for i in -23..24 {
        for j in -23..24 {
            for k in -23..24 {
                if !exts.contains(&(i, j, k)) {
                    hs.insert((i, j, k));
                }
            }
        }
    }
    let mut ans = hs.len() * 6;
    for pt in hs.iter() {
        for d in &dirs {
            let n_pt = add_vec3(*pt, *d);
            if hs.contains(&n_pt) {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
