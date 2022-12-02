fn main()
{
    let mut v = Vec::new();
    'outer: loop
    {
        let mut sum = 0i32;
        loop
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();
            if s == "-1"
            {
                break 'outer;
            }
            if s == ""
            {
                break;
            }
            let n: i32 = s.trim().parse().unwrap();
            sum += n;
        }
        v.push(sum);
    }
    v.sort();
    v.reverse();
    let s = v[0] + v[1] + v[2];
    println!("{}", s.to_string());
}