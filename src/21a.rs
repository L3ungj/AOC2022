fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn rd_int(s: &String) -> i64 {
    let end = s.find(' ');
    if end == None {
        return s.parse().unwrap();
    }
    s[..end.unwrap()].parse().unwrap()
}

use std::collections::HashMap;

fn sol(mk: String, mp: &HashMap<String, String>) -> i64 {
    let tokens : Vec<String> = mp[&mk].split(' ').map(String::from).collect();
    if tokens.len() == 1 {
        return rd_int(&tokens[0]);
    }
    match &*tokens[1] {
        "+" => sol(tokens[0].clone(), mp) + sol(tokens[2].clone(), mp),
        "-" => sol(tokens[0].clone(), mp) - sol(tokens[2].clone(), mp),
        "*" => sol(tokens[0].clone(), mp) * sol(tokens[2].clone(), mp),
        "/" => sol(tokens[0].clone(), mp) / sol(tokens[2].clone(), mp),
        _ => -1
    }
}

fn main() {
    let mut mp:HashMap<String, String> = HashMap::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let spl:Vec<String> = s.split(": ").map(String::from).collect();
        mp.insert(spl[0].clone(), spl[1].clone());
    }
    let ans = sol("root".to_string(), &mp);
    println!("{}", ans);
}
