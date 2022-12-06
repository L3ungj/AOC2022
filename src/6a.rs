use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let mut v = VecDeque::<u8>::new();
    let mut i = 0;
    for c in s.bytes()
    {
        i += 1;
        v.push_back(c);
        if v.len() > 14
        {
            v.pop_front();
        }
        else
        {
            continue;
        }
        if v[0] == v[1] || v[1] == v[2] || v[2] == v[3] || v[0] == v[2] || v[1] == v[3] || v[0] == v[3]
        {
            continue;
        }
        else {
            println!("{}", i.to_string());
            break;
        }
    }
}
