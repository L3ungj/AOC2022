fn main()
{
    let mut v = Vec::<Vec::<i32>>::new();
    let mut i = 0;
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./"
        {
            break;
        }
        v.push(Vec::new());
        for c in s.bytes() {
            v[i].push(c as i32 - 48);
        }
        i += 1;
    }
    let n = i;
    let mut cnt = 0;
    for i in 0..n {
        for j in 0..n {
            let mut ok = false;
            let mut t_ok;

            t_ok = true;
            for k in 0..j {
                if v[i][k] >= v[i][j] {
                    t_ok = false;
                }
            }
            ok = ok || t_ok;

            t_ok = true;
            for k in j+1..n {
                if v[i][k] >= v[i][j] {
                    t_ok = false;
                }
            }
            ok = ok || t_ok;
            
            t_ok = true;
            for k in 0..i {
                if v[k][j] >= v[i][j] {
                    t_ok = false;
                }
            }
            ok = ok || t_ok;
            
            t_ok = true;
            for k in i+1..n {
                if v[k][j] >= v[i][j] {
                    t_ok = false;
                }
            }
            ok = ok || t_ok;

            if ok {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}