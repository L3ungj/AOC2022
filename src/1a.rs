fn main()
{
    let mut ma = -1i32;
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
            println!("{}", s);
            let n: i32 = s.trim().parse().unwrap();
            sum += n;
        }
        ma = std::cmp::max(ma, sum);
    }
    println!("{}", ma.to_string());
}