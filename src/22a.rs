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

fn walk(x: &mut usize, y: &mut usize, dir: &usize, board: &Vec<Vec<char>>, edge: &Vec<Vec<Vec<usize>>>) {
    let mut new_x;
    let mut new_y;
    match dir {
        0 => {
            new_x = *x;
            new_y = *y + 1;
        }
        1 => {
            new_x = *x + 1;
            new_y = *y;
        }
        2 => {
            new_x = *x;
            new_y = *y - 1;
        }
        3 => {
            new_x = *x - 1;
            new_y = *y;
        },
        _ => panic!("Invalid direction"),
    }
    // fall outside
    if !(new_x > 0
        && new_x <= board.len()
        && new_y > 0
        && new_y <= board[0].len()
        && board[new_x - 1][new_y - 1] != ' ')
    {
        if *dir == 1 || *dir == 3 {
            new_x = edge[*dir][*x - 1][*y - 1] + 1;
        } else if *dir == 0 || *dir == 2 {
            new_y = edge[*dir][*x - 1][*y - 1] + 1;
        }
    }
    // println!("{} {} {}", new_x, new_y, edge[*dir][*x - 1][*y - 1] + 1);
    if board[new_x - 1][new_y - 1] == '#' {
        return;
    }
    *x = new_x;
    *y = new_y;
}

fn main() {
    let mut m = 0;
    let mut board: Vec<Vec<char>> = vec![];
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        if s == "" {
            break 'input_loop;
        }
        board.push(s.chars().collect());
        m = std::cmp::max(m, s.len());
    }
    let n = board.len();
    for i in 0..n {
        board[i].resize(m, ' ');
    }
    let mut edge: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; m]; n]; 4];
        for i in 0..n {
            for j in 0..m {
                edge[1][i][j] = i;
                edge[3][i][j] = i;
                edge[0][i][j] = j;
                edge[2][i][j] = j;
            }
        }
    // Direction is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
    for j in 0..m {
        for i in 1..n {
            if board[i - 1][j] == ' ' {
                edge[1][i][j] = i;
            } else {
                edge[1][i][j] = edge[1][i - 1][j];
            }
        }
        for i in (0..n - 1).rev() {
            if board[i + 1][j] == ' ' {
                edge[3][i][j] = i;
            } else {
                edge[3][i][j] = edge[3][i + 1][j];
            }
        }
    }
    for i in 0..n {
        for j in 1..m {
            if board[i][j - 1] == ' ' {
                edge[0][i][j] = j;
            } else {
                edge[0][i][j] = edge[0][i][j - 1];
            }
        }
        for j in (0..m - 1).rev() {
            if board[i][j + 1] == ' ' {
                edge[2][i][j] = j;
            } else {
                edge[2][i][j] = edge[2][i][j + 1];
            }
        }
    }
    let mut moves = String::new();
    rdln(&mut moves);
    let moves_spl: Vec<i32> = moves
        .split(['L', 'R'])
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();
    let rots: Vec<char> = moves.chars().filter(|&c| c == 'L' || c == 'R').collect();
    let mut dir = 0;
    let mut x = 1;
    let mut y = 1;
    for j in 0..m {
        if board[0][j] != ' ' {
            y = j + 1;
            break;
        }
    }
    for step in 0..rots.len() {
        for _ in 0..moves_spl[step] {
            walk(&mut x, &mut y, &dir, &board, &edge);
        }
        match rots[step] {
            'L' => dir = (dir + 3) % 4,
            'R' => dir = (dir + 1) % 4,
            _ => panic!("Invalid rotation"),
        }
        // println!("{} {} {}", x, y, dir);
    }
    for _ in 0..moves_spl[rots.len()] {
        walk(&mut x, &mut y, &dir, &board, &edge);
    }
    // println!("{} {} {}", x, y, dir);
    println!("{}", x*1000 + y*4 + dir);
}
