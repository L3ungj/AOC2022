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
        let mut s1 = String::new();
        std::io::stdin().read_line(&mut s1).unwrap();
        s1 = s1.trim().to_string();
        if s1 == "@#-3aV[./"
        {
            break;
        }
        let mut s2 = String::new();
        std::io::stdin().read_line(&mut s2).unwrap();
        s2 = s2.trim().to_string();
        let mut s3 = String::new();
        std::io::stdin().read_line(&mut s3).unwrap();
        s3 = s3.trim().to_string();

        let mut v1 = Vec::new();
        v1.resize(60, 0);
        for c in s1.chars()
        {
            v1[ord(c) as usize] += 1;
        }
        let mut v2 = Vec::new();
        v2.resize(60, 0);
        for c in s2.chars()
        {
            v2[ord(c) as usize] += 1;
        }
        let mut v3 = Vec::new();
        v3.resize(60, 0);
        for c in s3.chars()
        {
            v3[ord(c) as usize] += 1;
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