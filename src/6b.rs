use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let mut v = VecDeque::<u8>::new();
    let mut cur = 0;
    for c in s.bytes()
    {
        cur += 1;
        v.push_back(c);
        if v.len() > 14
        {
            v.pop_front();
        }
        else
        {
            continue;
        }
        let mut ok = true;
        let mut i = 0;
        while i < 14
        {
            let mut j = i+1;
            while j < 14
            {
                if v[i] == v[j]
                {
                    ok = false;
                }
                j += 1;
            }
            i += 1;
        }
        if ok{
            println!("{}", cur.to_string());
            break;
        }
    }
}
