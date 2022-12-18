use std::collections::HashMap;

type ll = i64;
type vi = Vec<ll>;

fn rdln(s: &mut String) -> bool {
    std::io::stdin().read_line(s).unwrap();
    *s = (*s).trim().to_string();
    if *s == "@#-3aV[./" {
        return true;
    }
    return false;
}

fn get_valve(s: &String, mp_valve: &mut HashMap<String, usize>, _cnt: &mut usize) -> usize {
    if mp_valve.contains_key(s) {
        return mp_valve[s];
    }
    mp_valve.insert(s.clone(), *_cnt);
    *_cnt += 1;
    return mp_valve[s];
}

fn dfs(u: usize, step: ll, vis: &Vec<bool>, dist: &Vec<Vec<ll>>, fr: &Vec<ll>) -> ll {
    let n = dist.len();
    let mut ma = 0;
    // println!("dfsing {}", u);
    for v in 0..n {
        if (*fr)[v] == 0 || v == u || vis[v] {
            continue;
        }
        if step + (*dist)[u][v] > 30 {
            continue;
        }
        let mut n_vis = vis.clone();
        let b_t_ref = n_vis.get_mut(v).unwrap();
        *b_t_ref = true;
        let n_step = step + (*dist)[u][v] + 1; // go there and open the valve
        let mut cur = dfs(v, n_step, &n_vis, dist, fr);
        cur += (*fr)[v] * (30 - n_step);
        if ma < cur {
            ma = cur;
        }
    }
    // println!("{} at {} returns {}", u, step, ma);
    // println!("best for {} is {}", u, best);
    return ma;
}

fn main() {
    let mut mp_valve: HashMap<String, usize> = HashMap::new();
    let mut _cnt = 0;
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); 100];
    let mut fr: Vec<ll> = vec![0; 100];
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let tokens: Vec<String> = s.split(';').map(String::from).collect();
        let valve_pos = s.find("Valve ").unwrap() + 6;
        let valve: String = s[valve_pos..valve_pos + 2].to_string();
        let flow_rate_pos = tokens[0].find("rate=").unwrap() + 5;
        let flow_rate = tokens[0][flow_rate_pos..].to_string();
        let to_valves_pos_w = s.find("valves ");
        let to_valves_pos;
        if to_valves_pos_w == None {
            to_valves_pos = s.find("valve ").unwrap() + 6;
        } else {
            to_valves_pos = to_valves_pos_w.unwrap() + 7;
        }
        let to_valves = s[to_valves_pos..].to_string();
        let u = get_valve(&valve, &mut mp_valve, &mut _cnt);
        for v_s in to_valves.split(", ") {
            let v = get_valve(&v_s.to_string(), &mut mp_valve, &mut _cnt);
            g[u].push(v);
        }
        // print!("{} {}: ", u, flow_rate);
        // for v in &G[u] {
        //     print!("{} ", v);
        // }
        // println!("");
        let fr_ref = fr.get_mut(u).unwrap();
        *fr_ref = flow_rate.parse().unwrap();
    }
    let n = _cnt;
    let mut dist: Vec<Vec<ll>> = vec![vec![1000000; n]; n];
    //flyod warshall
    for u in 0..n {
        for v in &g[u] {
            let dist_ref = dist.get_mut(u).unwrap().get_mut(*v).unwrap();
            *dist_ref = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    let d1 = dist[i][k].clone();
                    let d2 = dist[k][j].clone();
                    let dist_ref = dist.get_mut(i).unwrap().get_mut(j).unwrap();
                    *dist_ref = d1 + d2;
                }
            }
        }
    }
    let st_valve = get_valve(&"AA".to_string(), &mut mp_valve, &mut _cnt);
    let vis = vec![false; n];
    let mut ma = dfs(st_valve, 0, &vis, &dist, &fr);
    let mut idxs: Vec<usize> = Vec::new();
    for i in 0..n {
        if fr[i] > 0 {
            idxs.push(i);
        }
    }
    let m = idxs.len();
    // println!("m {}", m);
    for msk in 0..(1 << m) {
        // if msk % 1000 == 0 {
        //     println!("{}", msk);
        // }
        let mut vis1 = vec![false; n];
        let mut vis2 = vec![false; n];
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for i in 0..m {
            if (msk & (1 << i)) > 0 {
                let idx = idxs[i];
                vis1[idx] = true;
                cnt1 += 1;
            } else {
                let idx = idxs[i];
                vis2[idx] = true;
                cnt2 += 1;
            }
        }
        // to save time
        if cnt1 >= 2 * cnt2 || cnt2 >= 2 * cnt1 {
            continue;
        }
        // let mut vis1 = vec![false; n];
        let ans1 = dfs(st_valve, 4, &vis1, &dist, &fr);
        let ans2 = dfs(st_valve, 4, &vis2, &dist, &fr);
        // println!("{} {} {}", msk, ans1, ans2);
        ma = std::cmp::max(ma, ans1 + ans2);
    }
    println!("{}", ma);
}
