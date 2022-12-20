fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
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
    let mut ori_v = vec![];
    let mut iter = 0;
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        ori_v.push((rd_int(&s) as i64 * 811589153, iter));
        iter += 1;
    }
    let n = ori_v.len();
    let n_1 = (n - 1) as i64;
    let mut v = ori_v.clone();
    for _ in 0..10 {
        for a in &ori_v {
            let pos = v.iter().position(|&x| x == *a).unwrap();
            // println!("{} {}", pos, *a);
            let shift = ((a.0 % n_1) + n_1) % n_1;
            let new_pos = (pos + shift as usize) % n_1 as usize;
            // let new_pos = (pos + shift as usize) % n;
            v.remove(pos);
            if new_pos == 0 {
                v.push(*a);
            } else {
                v.insert(new_pos, *a);
            }
            // for i in 0..n {
            //     print!("{} ", v[i]);
            // }
            // println!("");
        }
    }
    let mut sum = 0;
    let zero_pos = v.iter().position(|&x| x.0 == 0).unwrap();
    for b in vec![1000, 2000, 3000] {
        // println!("{} {}", zero_pos, v[(zero_pos + b) % n]);
        sum += v[(zero_pos + b) % n].0;
    }
    println!("{}", sum);
}
