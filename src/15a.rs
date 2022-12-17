use std::collections::HashSet;

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
    let req: ll = 2000000;
    let mut rm_hs:HashSet<ll> = HashSet::new();
    let mut segs: Vec<seg> = Vec::new();
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
        let r = (x - bx).abs() + (y - by).abs();
        let dy = (req - y).abs();
        let dx_max = r - dy;
        if by == req {
            rm_hs.insert(bx);
        }
        if dx_max >= 0 {
            segs.push((x-dx_max, x+dx_max));
        }
    }
    let mut ma: ll = -99999999999;
    for sg in segs {
        
    }
    println!("{}", hs.len() - rm_hs.len());
}
