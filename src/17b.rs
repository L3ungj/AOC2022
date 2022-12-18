fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

use std::collections::HashSet;
type pt = (i64, i64);

fn get_rocks(t: i64) -> Vec<pt> {
    match t {
        0 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        1 => vec![(1, 0), (1, 1), (1, 2), (2, 1), (0, 1)],
        2 => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        3 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        _ => vec![],
    }
}

fn add_pt(a: pt, b: pt) -> pt {
    (a.0 + b.0, a.1 + b.1)
}

struct Rock {
    indiv_rocks: Vec<pt>,
    pos: pt,
}

impl Rock {
    fn new(pos: pt, t: i64) -> Rock {
        Rock {
            pos: pos,
            indiv_rocks: get_rocks(t % 5),
        }
    }
    fn check_collision(&self, hs: &HashSet<pt>) -> bool {
        for r in &self.indiv_rocks {
            let new_pt = add_pt(self.pos, *r);
            if hs.contains(&new_pt) || new_pt.0 < 0 || new_pt.1 < 0 || new_pt.1 > 6 {
                return true;
            }
        }
        return false;
    }
    fn add_to_hs(&self, hs: &mut HashSet<pt>) {
        for r in &self.indiv_rocks {
            hs.insert(add_pt(self.pos, *r));
        }
    }
    fn update_cur_h(&self, cur_h: &mut i64) {
        for r in &self.indiv_rocks {
            let new_pt = add_pt(self.pos, *r);
            if new_pt.0 + 1 > *cur_h {
                *cur_h = new_pt.0 + 1;
            }
        }
    }
}

fn update_hs(hs: &mut HashSet<pt>, cur_h: &i64) {
    let mut to_remove = vec![];
    for rock in hs.iter() {
        if rock.0 < *cur_h - 50 {
            to_remove.push(*rock);
        }
    }
    for rock in to_remove {
        hs.remove(&rock);
    }
}

fn drop(
    it: &usize,
    step: &mut i64,
    s: &String,
    hs: &mut HashSet<pt>,
    cur_h: &mut i64,
    rock: &mut Rock,
) {
    let c = s.chars().nth(*it as usize).unwrap();
    if c == '>' {
        rock.pos.1 += 1;
        if rock.check_collision(hs) {
            rock.pos.1 -= 1;
        }
    } else if c == '<' {
        rock.pos.1 -= 1;
        if rock.check_collision(hs) {
            rock.pos.1 += 1;
        }
    }
    rock.pos.0 -= 1;
    if rock.check_collision(&hs) {
        rock.pos.0 += 1;
        // println!("{} {} {}", step, rock.pos.0, rock.pos.1);
        rock.add_to_hs(hs);
        rock.update_cur_h(cur_h);
        update_hs(hs, cur_h);
        *step += 1;
        *rock = Rock::new((*cur_h + 3, 2), *step);
    }
}

fn check_equivalent(hs1: &HashSet<pt>, hs2: &HashSet<pt>, cur_h1: &i64, cur_h2: &i64) -> bool {
    if hs1.len() != hs2.len() {
        return false;
    }
    for rock in hs1.iter() {
        let nrock = (rock.0 - cur_h1 + cur_h2, rock.1);
        if !hs2.contains(&nrock) {
            return false;
        }
    }
    return true;
}

fn debug_hs(hs: &HashSet<pt>, cur_h: &i64) {
    let mut hs_v = hs.iter().collect::<Vec<_>>();
    hs_v.sort();
    for rock in hs_v {
        let nrock = (rock.0 - cur_h, rock.1);
        println!("{} {}", nrock.0, nrock.1);
    }
}

fn main() {
    let mut hs: HashSet<pt> = HashSet::new();
    let mut step = 0;
    let mut s = String::new();
    let mut cur_h = 0;
    rdln(&mut s);
    let n = s.len();
    let mut rock = Rock::new((cur_h + 3, 2), step);
    let mut st_it: usize = 0;
    for it in 0..1000000 {
        drop(&(it % n), &mut step, &s, &mut hs, &mut cur_h, &mut rock);
        if step == 500 {
            st_it = it;
            break;
        }
    }
    // after 500th stone, save the current states
    let st_hs = hs.clone();
    let st_step = step;
    let st_cur_h = cur_h;
    // debug_hs(&st_hs, &st_cur_h);
    let st_pos = rock.pos;
    // println!("{} {} {} {}", st_it, st_step, st_cur_h, st_hs.len());
    let mut cyc_h: i64 = 0;
    let mut cyc_size: i64 = 0;
    for it in (st_it + 1)..1000000000 {
        drop(&(it % n), &mut step, &s, &mut hs, &mut cur_h, &mut rock);
        if it % n == st_it % n {
            // println!("{} {} {} {}", it, step, cur_h, hs.len());
            // println!("{} {} {} {}", rock.pos.0, rock.pos.1, st_pos.0, st_pos.1);
            // debug_hs(&hs, &cur_h);
        }
        // check cycle
        if step % 5 == st_step % 5
            && it % n == st_it % n
            && rock.pos.1 == st_pos.1
            && rock.pos.0 - cur_h == st_pos.0 - st_cur_h
            && check_equivalent(&hs, &st_hs, &cur_h, &st_cur_h)
        {
            cyc_h = cur_h - st_cur_h;
            cyc_size = step - st_step;
            break;
        }
    }
    let req = 1000000000000;
    let cycs = (req - st_step) / cyc_size;
    cur_h = st_cur_h + cyc_h * cycs;
    step = st_step + cyc_size * cycs;
    // println!("found cycle {} {} {} {}", cyc_h, cyc_size, cur_h, step);
    //update hs
    hs.clear();
    for rock in st_hs.iter() {
        let nrock = (rock.0 - st_cur_h + cur_h, rock.1);
        hs.insert(nrock);
    }
    //update rock
    rock.pos.0 = cur_h + 3;
    assert!(check_equivalent(&hs, &st_hs, &cur_h, &st_cur_h));
    for it in (st_it + 1)..1000000000 {
        if step == req {
            println!("{}", cur_h);
            break;
        }
        drop(
            &(it % n),
            &mut step,
            &s,
            &mut hs,
            &mut cur_h,
            &mut rock,
        );
        // println!("{} {} {} {} {} {}", it, step, cur_h, hs.len(), rock.pos.0, rock.pos.1);
    }
}
