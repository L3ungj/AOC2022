// type ll = i64;
// type vi = Vec<ll>;

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

// add '\n' add end of input

fn is_endval(a:u8) -> bool{
    return a == ',' as u8 || a == ']' as u8;
}
fn read_val(v: &[u8], pos : &mut usize) -> i32{
    let mut s = String::new();
    while !is_endval(v[*pos]) {
        s.push(v[*pos] as char);
        *pos += 1;
    }
    return s.parse().unwrap();
}

fn main() {
    let mut cnt = 0;
    let mut input_idx = 0;
    'input_loop: loop {
        input_idx += 1;
        // println!("{}", input_idx);
        let mut ls = String::new();
        if rdln(&mut ls) {
            break 'input_loop;
        }
        let mut rs = String::new();
        rdln(&mut rs);
        let mut _dummy = String::new();
        rdln(&mut _dummy);
        let mut lpos = 0;
        let mut rpos = 0;
        let ls_ = ls.as_bytes();
        let rs_ = rs.as_bytes();
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
                    lvlst-=1;
                    if rs_[rpos] == ',' as u8 {
                        cnt += input_idx;
                        continue 'input_loop;
                    }
                    else {
                        rpos += 1;
                        continue;
                    }
                }
                if rvlst>0 {
                    rvlst -=1;
                    if ls_[lpos] == ',' as u8 {
                        continue 'input_loop;
                    }
                    else {
                        lpos += 1;
                        continue;
                    }
                }
                if ls_[lpos] == rs_[rpos] {
                    lpos += 1;
                    rpos += 1;
                }
                else if ls_[lpos] == ']' as u8{
                    cnt += input_idx;
                    continue 'input_loop;
                }
                else if rs_[rpos] == ']' as u8 {
                    continue 'input_loop;
                }
                continue;
            }
            if ls_[lpos] == ']' as u8 {
                cnt += input_idx;
                continue 'input_loop;
            }
            if rs_[rpos] == ']' as u8 {
                continue 'input_loop;
            }
            if ls_[lpos] != '[' as u8 && rs_[rpos] != '[' as u8 {
                let lval = read_val(ls_, &mut lpos);
                let rval = read_val(rs_, &mut rpos);
                if lval < rval {
                    cnt += input_idx;
                    continue 'input_loop;
                }
                else if lval > rval {
                    continue 'input_loop;
                }
                continue;
            }
            if ls_[lpos] == '[' as u8 {
                lpos += 1;
                rvlst += 1;
            }
            else if rs_[rpos] == '[' as u8 {
                rpos += 1;
                lvlst += 1;
            }

        }
    }
    println!("{}", cnt);
}
