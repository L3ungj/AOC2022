use std::collections::HashMap;
fn main() {
    let mut mp = HashMap::<String, Vec<String>>::new();
    let mut mp_sz = HashMap::<String, i32>::new();
    let mut cur_dir: String = "".to_string();
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./" {
            break;
        }
        // println!("{}", s);
        let tokens: Vec<String> = s.split(' ').map(String::from).collect();
        if tokens[0] == "$" {
            if tokens[1] == "cd" {
                if tokens[2] == ".." {
                    let l = cur_dir.len();
                    let pos = cur_dir[..l-1].rfind('/').unwrap() + 1;
                    cur_dir = cur_dir[..pos].to_string();
                }
                else if tokens[2] == "/" {
                    cur_dir = tokens[2].clone();
                }
                else {
                    cur_dir = format!("{}{}/", cur_dir, tokens[2].clone());
                }
                // println!("{}", cur_dir);
            } else {
                mp.insert(cur_dir.clone(), Vec::new());
                mp_sz.insert(cur_dir.clone(), 0);
            }
        } else {
            if tokens[0] == "dir" {
                let v = mp.get_mut(&cur_dir).unwrap();
                v.push(tokens[1].clone());
            } else {
                let val = mp_sz.get_mut(&cur_dir).unwrap();
                *val += tokens[0].parse::<i32>().unwrap();
            }
        }
    }
    let mut sum = 0;
    for a in &mp {
        let sz = get_sz(&mp, &mp_sz, &a.0);
        if sz <= 100000 {
            sum += sz;
        }
    }
    println!("{}", sum);
}

fn get_sz(mp: &HashMap<String, Vec<String>>, mp_sz : &HashMap<String, i32>, cur_dir: &String)->i32 {
    // println!("getting sz {}", cur_dir);
    let mut sum = mp_sz[cur_dir];
    for dir_name in &mp[cur_dir] {
        let dir_path = format!("{}{}/",cur_dir, dir_name);
        sum += get_sz(mp, mp_sz, &dir_path);
    }
    return sum;
}
