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
        let _s = &*s;
        sum += match _s{
            "A X"=>4,
            "A Y"=>8,
            "A Z"=>3,
            "B X"=>1,
            "B Y"=>5,
            "B Z"=>9,
            "C X"=>7,
            "C Y"=>2,
            "C Z"=>6,
            _=>-1
        }
    }
    println!("{}", sum.to_string());
}