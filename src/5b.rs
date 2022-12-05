// Very funny.
//         [F] [Q]         [Q]        
// [B]     [Q] [V] [D]     [S]        
// [S] [P] [T] [R] [M]     [D]        
// [J] [V] [W] [M] [F]     [J]     [J]
// [Z] [G] [S] [W] [N] [D] [R]     [T]
// [V] [M] [B] [G] [S] [C] [T] [V] [S]
// [D] [S] [L] [J] [L] [G] [G] [F] [R]
// [G] [Z] [C] [H] [C] [R] [H] [P] [D]
//  1   2   3   4   5   6   7   8   9 
// remove the above and '\n' from the input

fn main()
{
    // let mut stacks = Vec::<Vec::<char>>::new();
    let mut stacks = vec![
        vec!['G', 'D', 'V', 'Z', 'J', 'S', 'B'],
        vec!['Z', 'S', 'M', 'G', 'V', 'P'],
        vec!['C', 'L', 'B', 'S', 'W', 'T', 'Q', 'F'],
        vec!['H', 'J', 'G', 'W', 'M', 'R', 'V', 'Q'],
        vec!['C', 'L', 'S', 'N', 'F', 'M', 'D'],
        vec!['R', 'G', 'C', 'D'],
        vec!['H', 'G', 'T', 'R', 'J', 'D', 'S', 'Q'],
        vec!['P', 'F', 'V'],
        vec!['D', 'R', 'S', 'T', 'J'],
    ];
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        if s == "@#-3aV[./"
        {
            break;
        }
        let line_split : Vec<&str> = s.split(" ").collect(); 
        let num = line_split[1].parse::<i32>().unwrap();
        let from = line_split[3].parse::<usize>().unwrap();
        let to = line_split[5].parse::<usize>().unwrap();
        let mut i :i32 = 0;
        let mut tmp_v = Vec::<char>::new();
        while i < num
        {
            let t = stacks[from-1].pop().unwrap();
            tmp_v.push(t);
            i += 1;
        }
        i=0;
        while i<num
        {
            let t = tmp_v.pop().unwrap();
            stacks[to-1].push(t);
            i += 1;
        }
    }
    let mut i:usize = 0;
    while i < 9
    {
        print!("{}", stacks[i].last().unwrap());
        i += 1;
    }
}