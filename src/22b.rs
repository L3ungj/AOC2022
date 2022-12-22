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

// _01
// _2_
// 34_
// 5__

// Direction is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
// EDGE_RULES[i][dir] = (to, dir_of_to)
const EDGE_RULES: [[(usize, usize); 4]; 6] = [
    [(1, 3), (2, 3), (3, 1), (5, 2)],
    [(4, 1), (2, 2), (0, 3), (5, 3)],
    [(1, 0), (4, 3), (3, 0), (0, 3)],
    [(4, 3), (5, 3), (0, 1), (2, 2)],
    [(1, 1), (5, 2), (3, 3), (2, 3)],
    [(4, 0), (1, 3), (0, 0), (3, 3)],
];
const BOX_LEN: usize = 50;
const ST_POS: [(usize, usize); 6] = [
    (0, BOX_LEN),
    (0, BOX_LEN * 2),
    (BOX_LEN, BOX_LEN),
    (BOX_LEN * 2, 0),
    (BOX_LEN * 2, BOX_LEN),
    (BOX_LEN * 3, 0),
];

fn rev(a: &mut usize) {
    *a = BOX_LEN - 1 - *a;
}

fn walk_edge(from: &usize, dir: &usize, x: &mut usize, y: &mut usize) -> (usize, usize) {
    let (to, dir_of_to) = EDGE_RULES[*from][*dir];
    *x -= 1;
    *y -= 1;
    match dir_of_to {
        3 => match *dir {
            0 => *y = 0,
            1 => *x = 0,
            2 => *y = BOX_LEN - 1,
            3 => *x = BOX_LEN - 1,
            _ => panic!("Invalid direction"),
        },
        0 => {
            std::mem::swap(x, y);
            match *dir {
                0 => *x = BOX_LEN - 1,
                1 => rev(y),
                2 => *x = 0,
                3 => rev(y),
                _ => panic!("Invalid direction"),
            }
        }
        1 => match *dir {
            0 => rev(x),
            1 => rev(y),
            2 => rev(x),
            3 => rev(y),
            _ => panic!("Invalid direction"),
        },
        2 => {
            std::mem::swap(x, y);
            match *dir {
                0 => rev(x),
                1 => *y = BOX_LEN - 1,
                2 => rev(x),
                3 => *y = 0,
                _ => panic!("Invalid direction"),
            }
        }
        _ => panic!("Invalid direction"),
    }
    *x += 1;
    *y += 1;
    let new_dir = (*dir + 4 - dir_of_to + 3) % 4;
    (to, new_dir)
}

fn corr(box_: &usize, x: &usize, y: &usize) -> (usize, usize) {
    (ST_POS[*box_].0 + x - 1, ST_POS[*box_].1 + y - 1)
}

fn walk(box_: &mut usize, x: &mut usize, y: &mut usize, dir: &mut usize, board: &Vec<Vec<char>>) {
    let mut new_x;
    let mut new_y;
    let mut new_dir = *dir;
    let mut new_box = *box_;
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
        }
        _ => panic!("Invalid direction"),
    }
    // fall outside of box
    if !(new_x > 0 && new_x <= BOX_LEN && new_y > 0 && new_y <= BOX_LEN) {
        new_x = *x;
        new_y = *y;
        (new_box, new_dir) = walk_edge(box_, dir, &mut new_x, &mut new_y);
    }
    // println!("{} {} {}", new_x, new_y, edge[*dir][*x - 1][*y - 1] + 1);
    let (corr_x, corr_y) = corr(&new_box, &new_x, &new_y);
    if board[corr_x][corr_y] == '#' {
        return;
    }
    *x = new_x;
    *y = new_y;
    *dir = new_dir;
    *box_ = new_box;
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
    // Direction is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
    let mut moves = String::new();
    rdln(&mut moves);
    let moves_spl: Vec<i32> = moves
        .split(['L', 'R'])
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect();
    let rots: Vec<char> = moves.chars().filter(|&c| c == 'L' || c == 'R').collect();
    let mut dir = 0;
    let mut box_ = 0;
    let mut x = 1;
    let mut y = 1;
    for step in 0..rots.len() {
        for _ in 0..moves_spl[step] {
            walk(&mut box_, &mut x, &mut y, &mut dir, &board);
        }
        match rots[step] {
            'L' => dir = (dir + 3) % 4,
            'R' => dir = (dir + 1) % 4,
            _ => panic!("Invalid rotation"),
        }
        // println!("{} {} {}", x, y, dir);
    }
    for _ in 0..moves_spl[rots.len()] {
        walk(&mut box_, &mut x, &mut y, &mut dir, &board);
    }
    // println!("{} {} {}", x, y, dir);
    let (corr_x, corr_y) = corr(&box_, &x, &y);
    println!("{}", (corr_x + 1) * 1000 + (corr_y + 1) * 4 + dir);
}
