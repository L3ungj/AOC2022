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
    let mut ma = -1;
    for i in 0..n {
        for j in 0..n {
            let mut prod = 1;
            let mut cnt;

            cnt = 0;
            for k in (0..j).rev() {
                cnt += 1;
                if v[i][k] >= v[i][j] {
                    break;
                }
            }
            prod *= cnt;

            cnt = 0;
            for k in j+1..n {
                cnt += 1;
                if v[i][k] >= v[i][j] {
                    break;
                }
            }
            prod *= cnt;
            
            cnt = 0;
            for k in (0..i).rev() {
                cnt += 1;
                if v[k][j] >= v[i][j] {
                    break;
                }
            }
            prod *= cnt;
            
            cnt = 0;
            for k in i+1..n {
                cnt += 1;
                if v[k][j] >= v[i][j] {
                    break;
                }
            }
            prod *= cnt;

            ma = std::cmp::max(ma, prod);
        }
    }
    println!("{}", ma);
}