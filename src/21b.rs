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

const UNKNOWN: i64 = 9999999997;

fn sol(mk: String, mp: &HashMap<String, String>, val: &mut HashMap<String, i64>) -> i64 {
    let tokens: Vec<String> = mp[&mk].split(' ').map(String::from).collect();
    if mk == "humn" {
        val.insert(mk.clone(), UNKNOWN);
        return UNKNOWN;
    }
    if tokens.len() == 1 {
        val.insert(mk.clone(), rd_int(&tokens[0]));
        return rd_int(&tokens[0]);
    }
    let lval = sol(tokens[0].clone(), mp, val);
    let rval = sol(tokens[2].clone(), mp, val);
    let res;
    if lval == UNKNOWN || rval == UNKNOWN {
        res = UNKNOWN;
    } else {
        res = match &*tokens[1] {
            "+" => lval + rval,
            "-" => lval - rval,
            "*" => lval * rval,
            "/" => lval / rval,
            _ => UNKNOWN,
        };
    }
    val.insert(mk.clone(), res);
    return res;
}

fn sol2(mk: String, req: i64, mp: &HashMap<String, String>, val: HashMap<String, i64>) {
    if mk == "humn" {
        println!("{}", req);
        return;
    }
    let tokens: Vec<String> = mp[&mk].split(' ').map(String::from).collect();
    let lval = val[&tokens[0]];
    let rval = val[&tokens[2]];
    // println!("{} {} {} {}", mk, req, lval, rval);
    if lval == UNKNOWN {
        match &*tokens[1] {
            "+" => sol2(tokens[0].clone(), req - rval, mp, val),
            "-" => sol2(tokens[0].clone(), req + rval, mp, val),
            "*" => sol2(tokens[0].clone(), req / rval, mp, val),
            "/" => sol2(tokens[0].clone(), req * rval, mp, val),
            _ => (),
        }
    } else if rval == UNKNOWN {
        match &*tokens[1] {
            "+" => sol2(tokens[2].clone(), req - lval, mp, val),
            "-" => sol2(tokens[2].clone(), lval - req, mp, val),
            "*" => sol2(tokens[2].clone(), req / lval, mp, val),
            "/" => sol2(tokens[2].clone(), lval / req, mp, val),
            _ => (),
        }
    }
}

fn main() {
    let mut mp: HashMap<String, String> = HashMap::new();
    let mut val: HashMap<String, i64> = HashMap::new();
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let spl: Vec<String> = s.split(": ").map(String::from).collect();
        if spl[0] == "root" {
            let mut new_expr = spl[1].clone();
            new_expr.replace_range(5..6, "-");
            mp.insert(spl[0].clone(), new_expr.clone());
        } else {
            mp.insert(spl[0].clone(), spl[1].clone());
        }
    }
    sol("root".to_string(), &mp, &mut val);
    sol2("root".to_string(), 0, &mp, val);
}
