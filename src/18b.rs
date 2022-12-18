type Vec3 = (i64, i64, i64);

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

fn dfs_ext(pos: Vec3, vis: &mut HashSet<Vec3>, hs: &HashSet<Vec3>, exts: &mut HashSet<Vec3>) -> bool{
    if pos.0 < -22 || pos.0 > 22 || pos.1 < -22 || pos.1 > 22 || pos.2 < -22 || pos.2 > 22 {
        return true;
    }    
    // println!("{} {} {}", pos.0, pos.1, pos.2);
    let dirs = vec![(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)];
    let mut ret:bool = false;
    for d in &dirs {
        let n_pos = add_vec3(pos, *d);
        if !vis.contains(&n_pos) && !hs.contains(&n_pos) {
            vis.insert(n_pos);
            ret |= dfs_ext(n_pos, vis, hs, exts);
            break;
        }
    }
    if !ret {
        for pt in vis.iter() {
            exts.insert(*pt);
        }
    }
    ret
}

fn main() {
    let mut hs:HashSet<Vec3> = HashSet::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let raw_pt : Vec<i64> = s.split(',').map(|x| x.parse().unwrap()).collect();
        let pt = (raw_pt[0], raw_pt[1], raw_pt[2]);
        hs.insert(pt);
    }
    let dirs : Vec<Vec3> = vec![(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)];
    let mut exts:HashSet<Vec3> = HashSet::new();
    dfs_ext((-22, -22, -22), &mut HashSet::new(), &hs, &mut exts);
    for i in -22..23 {
        for j in -22..23 {
            for k in -22..23 {
                let pt = (i, j, k);
                if !exts.contains(&pt) {
                    let mut vis:HashSet<Vec3> = HashSet::new();
                    vis.insert(pt);
                    dfs(pt, &vis, &mut hs);
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
