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

fn snafu_to_dec(s: &String) -> i64 {
    let mut ret = 0;
    let mut cur = 1;
    for c in s.chars().rev() {
        ret += match c {
            '=' => -2,
            '-' => -1,
            _ => c as i64 - '0' as i64,
        } * cur;
        cur *= 5;
    }
    ret
}

fn dec_to_snafu(n: i64) -> String {
    let mut ret_vec = Vec::new();
    let mut cur = n;
    while cur != 0 {
        let rem = cur % 5;
        cur /= 5;
        ret_vec.push(rem);
    }
    let mut carry = 0;
    for c in &mut ret_vec {
        *c += carry;
        carry = 0;
        if *c <= 2 {
            continue;
        }
        *c -= 5;
        carry += 1;
    }
    if carry != 0 {
        ret_vec.push(carry);
    }
    let ret = ret_vec
        .iter()
        .rev()
        .map(|x| match x {
            -2 => '=',
            -1 => '-',
            _ => (x + '0' as i64) as u8 as char,
        })
        .collect();
    ret
}

fn main() {
    let mut sum = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        sum += snafu_to_dec(&s);
    }
    println!("{}", dec_to_snafu(sum));
}
