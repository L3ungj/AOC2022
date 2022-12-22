fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim_end().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn rd_int(s: &String) -> i32 {
    let end = s.find(' ');
    if end == None {
        return s.parse().unwrap();
    }
    s[..end.unwrap()].parse().unwrap()
}

fn main() {
    let mut sum = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        sum += match &*s {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => -1,
        }
    }
    println!("{}", sum);
}
