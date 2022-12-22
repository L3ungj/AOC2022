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

fn ord(c: char) -> usize
{
    let a = c as usize;
    if a >= 97 {return a - 96};
    return a-64 + 26;
}

fn main()
{
    let mut sum = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let n = s.len();
        let h : usize = n/2;
        let s1 = &s[..h];
        let s2 = &s[h..];
        let mut v = Vec::new();
        v.resize(60, 0);
        for c in s1.chars()
        {
            v[ord(c)] += 1;
        }
        let mut ans = 0;
        for c in s2.chars()
        {
            if v[ord(c)] > 0
            {
                // println!("{}",c);
                ans = ord(c);
                // println!("{}",ans.to_string());
                break;
            }
        }
        sum += ans;
    }
    println!("{}", sum);
}