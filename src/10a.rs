fn main()
{
    let mut sum = 0;
    let mut step = 0;
    let mut cur = 1;
    let mut pass = false;
    let mut end_pass = 0;
    let mut end_pass2 = 0;
    loop {
        if step < 230 && ((step-20) % 40 == 0) {
            // println!("{} {} {} {}", cur, step, pass, end_pass);
            sum += cur * step;
        }
        cur += end_pass2;
        end_pass2 = 0;
        step += 1;
        if pass {
            end_pass2 = end_pass;
            end_pass = 0;
            pass = false;
            continue;
        }
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./"
        {
            break;
        }
        let tokens :Vec<String> = s.split(' ').map(String::from).collect();
        if tokens[0] == "noop" {
            continue;
        }
        end_pass = tokens[1].parse::<i32>().unwrap();
        pass = true;
    }
    // println!("{}", step);
    println!("{}", sum);
}