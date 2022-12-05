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
        let split_comma: Vec<&str> = s.split(",").collect();
        let s1 = split_comma[0];
        let s2 = split_comma[1];
        let s1_split: Vec<&str> = s1.split("-").collect();
        let s2_split: Vec<&str> = s2.split("-").collect();
        let l1 = s1_split[0].parse::<i32>().unwrap();let r1 = s1_split[1].parse::<i32>().unwrap();
        let l2 = s2_split[0].parse::<i32>().unwrap();let r2 = s2_split[1].parse::<i32>().unwrap();
        if (l1 >= l2 && l1 <= r2) || (r1 >= l2 && r1 <= r2) || (r2 >= l1 && r2 <= r1)
        {
            sum += 1;
        }
    }
    println!("{}", sum.to_string());
}