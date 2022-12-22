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
        let mut s1 = String::new();
        if rdln(&mut s1) {
            break 'input_loop;
        }
        let mut s2 = String::new();
        rdln(&mut s2);
        let mut s3 = String::new();
        rdln(&mut s3);

        let mut v1 = Vec::new();
        v1.resize(60, 0);
        for c in s1.chars()
        {
            v1[ord(c)] += 1;
        }
        let mut v2 = Vec::new();
        v2.resize(60, 0);
        for c in s2.chars()
        {
            v2[ord(c)] += 1;
        }
        let mut v3 = Vec::new();
        v3.resize(60, 0);
        for c in s3.chars()
        {
            v3[ord(c)] += 1;
        }

        let mut i = 0;
        while i < 60
        {
            if v1[i] > 0 && v2[i] > 0 && v3[i] > 0
            {
                break;
            }
            i += 1;
        }
        
        sum += i;
    }
    println!("{}", sum.to_string());
}