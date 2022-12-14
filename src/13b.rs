// type ll = i64;
// type vi = Vec<ll>;

use std::cmp::Ordering;

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

// add '\n' add end of input

fn is_endval(a: u8) -> bool {
    return a == ',' as u8 || a == ']' as u8;
}
fn read_val(v: &[u8], pos: &mut usize) -> i32 {
    let mut s = String::new();
    while !is_endval(v[*pos]) {
        s.push(v[*pos] as char);
        *pos += 1;
    }
    return s.parse().unwrap();
}

fn cmp_packet(s1: &String, s2: &String) -> Ordering {
    let mut lpos = 0;
    let mut rpos = 0;
    let ls_ = s1.as_bytes();
    let rs_ = s2.as_bytes();
    let mut lvlst = 0;
    let mut rvlst = 0;
    loop {
        // println!("{} {}", lpos, rpos);
        if ls_[lpos] == '[' as u8 && rs_[rpos] == '[' as u8 {
            lpos += 1;
            rpos += 1;
            continue;
        }
        if is_endval(ls_[lpos]) && is_endval(rs_[rpos]) {
            if lvlst > 0 {
                lvlst -= 1;
                if rs_[rpos] == ',' as u8 {
                    return Ordering::Less;
                } else {
                    rpos += 1;
                    continue;
                }
            }
            if rvlst > 0 {
                rvlst -= 1;
                if ls_[lpos] == ',' as u8 {
                    return Ordering::Greater;
                } else {
                    lpos += 1;
                    continue;
                }
            }
            if ls_[lpos] == rs_[rpos] {
                lpos += 1;
                rpos += 1;
            } else if ls_[lpos] == ']' as u8 {
                return Ordering::Less;
            } else if rs_[rpos] == ']' as u8 {
                return Ordering::Greater;
            }
            continue;
        }
        if ls_[lpos] == ']' as u8 {
            return Ordering::Less;
        }
        if rs_[rpos] == ']' as u8 {
            return Ordering::Greater;
        }
        if ls_[lpos] != '[' as u8 && rs_[rpos] != '[' as u8 {
            let lval = read_val(ls_, &mut lpos);
            let rval = read_val(rs_, &mut rpos);
            if lval < rval {
                return Ordering::Less;
            } else if lval > rval {
                return Ordering::Greater;
            }
            continue;
        }
        if ls_[lpos] == '[' as u8 {
            lpos += 1;
            rvlst += 1;
        } else if rs_[rpos] == '[' as u8 {
            rpos += 1;
            lvlst += 1;
        }
    }
}

fn main() {
    let mut v: Vec<String> = Vec::new();
    'input_loop: loop {
        // println!("{}", input_idx);
        let mut ls = String::new();
        if rdln(&mut ls) {
            break 'input_loop;
        }
        let mut rs = String::new();
        rdln(&mut rs);
        let mut _dummy = String::new();
        rdln(&mut _dummy);
        v.push(ls);
        v.push(rs);
    }
    v.push("[[2]]".to_string());
    v.push("[[6]]".to_string());
    v.sort_by(cmp_packet);
    let mut prod = 1;
    let mut idx = 0;
    for s in v {
        idx += 1;
        if s == "[[2]]" || s == "[[6]]" {
            prod *= idx;
        }
    }
    println!("{}", prod);
}
