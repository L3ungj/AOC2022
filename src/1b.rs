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

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut sum = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        if s == "" {
            v.push(sum);
            sum = 0;
            continue 'input_loop;
        }
        let n = rd_int(&s);
        sum += n;
    }
    v.push(sum);
    v.sort();
    v.reverse();
    println!("{}", v[0] + v[1] + v[2]);
}
