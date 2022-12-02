
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
        sum += match &s[..] {
            "A X"=>3,
            "A Y"=>4,
            "A Z"=>8,
            "B X"=>1,
            "B Y"=>5,
            "B Z"=>9,
            "C X"=>2,
            "C Y"=>6,
            "C Z"=>7,
            _=>-1
        }
    }
    println!("{}", sum.to_string());
}