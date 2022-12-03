fn ord(c: char) -> i32
{
    let a = c as i32;
    if a >= 97 {return a - 96};
    return a-64 + 26;
}

fn main()
{
    let mut sum = 0;
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "-1"
        {
            break;
        }
        let n = s.len();
        let h : usize = n/2;
        let s1 = &s[..h];
        let s2 = &s[h..];
        let mut v = Vec::new();
        v.resize(60, 0);
        for c in s1.chars()
        {
            v[ord(c) as usize] += 1;
        }
        let mut ans = -1;
        for c in s2.chars()
        {
            if v[ord(c) as usize] > 0
            {
                // println!("{}",c);
                ans = ord(c);
                // println!("{}",ans.to_string());
                break;
            }
        }
        sum += ans;
    }
    println!("{}", sum.to_string());
}