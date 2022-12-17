type ll = i64;
type vi = Vec<ll>;
type seg = (ll, ll);

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn main() {
    let mut v:Vec<(ll, ll,ll, ll)> = Vec::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let tokens: Vec<String> = s.split('=').map(String::from).collect();
        let xs = tokens[1][0..(tokens[1].find(',')).unwrap()].to_string();
        let x: ll = xs.parse().unwrap();
        let ys = tokens[2][0..(tokens[2].find(':')).unwrap()].to_string();
        let y: ll = ys.parse().unwrap();
        let bxs = tokens[3][0..(tokens[3].find(',')).unwrap()].to_string();
        let bx: ll = bxs.parse().unwrap();
        let bys = tokens[4].to_string();
        let by: ll = bys.parse().unwrap();
        v.push((x, y, bx, by));
    }
    let mut ans = 0;
    for req in 0..4000001 {
        let mut segs: Vec<seg> = Vec::new();
        for (x, y, bx, by) in &v {
            let r = (x - bx).abs() + (y - by).abs();
            let dy = (req - y).abs();
            let dx_max = r - dy;
            if dx_max >= 0 {
                segs.push((x-dx_max, x+dx_max));
            }
        }
        let mut ma: ll = -1;
        let mut st: ll = -1;
        segs.sort();
        for sg in segs {
            if st == -1 {
                if sg.0 > 0 {
                    assert_eq!(ans, 0);
                    ans = 0 * 4000000 + req;
                }
                st = sg.0;
                ma = sg.1;
            }
            if sg.0 > ma {
                assert_eq!(ans, 0);
                assert!(sg.0 - 1 == ma + 1);
                ans = (sg.0-1) * 4000000 + req;
                // println!("{} {}", sg.0-1, req);
                st = sg.0;
            }
            ma = std::cmp::max(ma, sg.1);
        }
        if ma < 4000000 {
            assert_eq!(ans, 0);
            ans = 4000000 * 4000000 + req;
        }
    }
    println!("{}", ans);
}
