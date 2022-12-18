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
