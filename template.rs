fn main()
{
    let mut sum = 0;
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./"
        {
            break;
        }
        
        sum += 0;
    }
    println!("{}", sum.to_string());
}