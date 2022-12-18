fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

use std::collections::HashSet;
type pt = (i32, i32);

fn get_rocks(t: i32) -> Vec<pt> {
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
    fn new(pos: pt, t: i32) -> Rock {
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
    fn update_cur_h(&self, cur_h: &mut i32) {
        for r in &self.indiv_rocks {
            let new_pt = add_pt(self.pos, *r);
            if new_pt.0 +1 > *cur_h {
                *cur_h = new_pt.0 + 1;
            }
        }
    }
}

fn main() {
    let mut hs: HashSet<pt> = HashSet::new();
    let mut step = 0;
    let mut s = String::new();
    let mut cur_h = 0;
    rdln(&mut s);
    let mut rock = Rock::new((cur_h + 3, 2), step);
    for it in 0..1000000000000 {
        let c = s.chars().nth((it % s.len()) as usize).unwrap();
        if it % 1000 == 0 {
            // println!("{} {} {} {}", it, step, cur_h, hs.len());
        }
        if step == 2022 {
            println!("{}", cur_h);
            return;
        }
        if c == '>' {
            rock.pos.1 += 1;
            if rock.check_collision(&hs) {
                rock.pos.1 -= 1;
            }
        } else if c == '<' {
            rock.pos.1 -= 1;
            if rock.check_collision(&hs) {
                rock.pos.1 += 1;
            }
        }
        rock.pos.0 -= 1;
        if rock.check_collision(&hs) {
            rock.pos.0 += 1;
            // println!("{} {} {}", step, rock.pos.0, rock.pos.1);
            rock.add_to_hs(&mut hs);
            rock.update_cur_h(&mut cur_h);
            step += 1;
            rock = Rock::new((cur_h + 3, 2), step);
        }
    }
}
