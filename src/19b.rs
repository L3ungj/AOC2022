use std::cmp::Ordering;

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

type item = (i32, i32, i32, i32);

fn my_cmp(a: &item, b: &item) -> Ordering {
    if a.3 > b.3
        || (a.3 == b.3 && a.2 > b.2)
        || (a.3 == b.3 && a.2 == b.2 && a.1 > b.1)
        || (a.3 == b.3 && a.2 == b.2 && a.1 == b.1 && a.0 > b.0)
    {
        return Ordering::Greater;
    }
    if a.3 == b.3 && a.2 == b.2 && a.1 == b.1 && a.0 == b.0 {
        return Ordering::Equal;
    }
    return Ordering::Less;
}

const MAX_BRANCHES: usize = 10;

fn upd(a: &mut Vec<item>, b: &item) {
    a.push(*b);
    //sort in descending order
    a.sort_by(|a, b| my_cmp(b, a));
    a.truncate(MAX_BRANCHES)
}

fn main() {
    let mut tot_ans = 1;
    let mut iter = 0;
    'input_loop: loop {
        iter += 1;
        if iter == 4 {
            break;
        }
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let tokens: Vec<String> = s.split("costs ").map(String::from).collect();
        let ore_cost = rd_int(&tokens[1]);
        let clay_cost = rd_int(&tokens[2]);
        let obsidian_cost = (rd_int(&tokens[3]), rd_int(&(tokens[3][10..].to_string())));
        let geode_cost = (rd_int(&tokens[4]), rd_int(&(tokens[4][10..].to_string())));
        let mut dp: Vec<Vec<Vec<Vec<Vec<Vec<item>>>>>> =
            vec![vec![vec![vec![vec![vec![(-1, -1, -1, -1); MAX_BRANCHES]; 11]; 11]; 11]; 11]; 33];
        dp[0][1][0][0][0][0] = (0, 0, 0, 0);
        for step in 0..32 {
            for ore_robot in 0..11 {
                for clay_robot in 0..11 {
                    for obsidian_robot in 0..11 {
                        for geode_robot in 0..11 {
                            for i in 0..MAX_BRANCHES {
                                if dp[step][ore_robot][clay_robot][obsidian_robot][geode_robot][i]
                                    == (-1, -1, -1, -1)
                                {
                                    continue;
                                }
                                let (ore, clay, obsidian, geode) =
                                    dp[step][ore_robot][clay_robot][obsidian_robot][geode_robot][i];
                                let (new_ore, new_clay, new_obsidian, new_geode) = (
                                    ore + ore_robot as i32,
                                    clay + clay_robot as i32,
                                    obsidian + obsidian_robot as i32,
                                    geode + geode_robot as i32,
                                );
                                if ore >= ore_cost && ore_robot + 1 < 11 {
                                    upd(
                                        &mut dp[step + 1][ore_robot + 1][clay_robot]
                                            [obsidian_robot][geode_robot],
                                        &(new_ore - ore_cost, new_clay, new_obsidian, new_geode),
                                    );
                                }
                                if ore >= clay_cost && clay_robot + 1 < 11 {
                                    upd(
                                        &mut dp[step + 1][ore_robot][clay_robot + 1]
                                            [obsidian_robot][geode_robot],
                                        &(new_ore - clay_cost, new_clay, new_obsidian, new_geode),
                                    );
                                }
                                if ore >= obsidian_cost.0
                                    && clay >= obsidian_cost.1
                                    && obsidian_robot + 1 < 11
                                {
                                    upd(
                                        &mut dp[step + 1][ore_robot][clay_robot]
                                            [obsidian_robot + 1][geode_robot],
                                        &(
                                            new_ore - obsidian_cost.0,
                                            new_clay - obsidian_cost.1,
                                            new_obsidian,
                                            new_geode,
                                        ),
                                    );
                                }
                                if ore >= geode_cost.0
                                    && obsidian >= geode_cost.1
                                    && geode_robot + 1 < 11
                                {
                                    upd(
                                        &mut dp[step + 1][ore_robot][clay_robot][obsidian_robot]
                                            [geode_robot + 1],
                                        &(
                                            new_ore - geode_cost.0,
                                            new_clay,
                                            new_obsidian - geode_cost.1,
                                            new_geode,
                                        ),
                                    );
                                }
                                upd(
                                    &mut dp[step + 1][ore_robot][clay_robot][obsidian_robot]
                                        [geode_robot],
                                    &(new_ore, new_clay, new_obsidian, new_geode),
                                );
                            }
                        }
                    }
                }
            }
        }
        let mut ans = (-1, -1, -1, -1);
        for ore_robot in 0..11 {
            for clay_robot in 0..11 {
                for obsidian_robot in 0..11 {
                    for geode_robot in 0..11 {
                        if my_cmp(
                            &ans,
                            &dp[32][ore_robot][clay_robot][obsidian_robot][geode_robot][0],
                        ) == Ordering::Less
                        {
                            ans = dp[32][ore_robot][clay_robot][obsidian_robot][geode_robot][0];
                        }
                    }
                }
            }
        }
        // println!("{}", ans.3);
        // println!("{} {} {} {}", ans.0, ans.1, ans.2, ans.3);
        tot_ans *= ans.3;
    }
    println!("{}", tot_ans);
}
