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

fn get_valve(s : &String, mp_valve : &mut HashMap<String, usize>, _cnt: &mut usize) -> usize{
    if mp_valve.contains_key(s) {
        return mp_valve[s];
    }
    let val_ref = mp_valve.get_mut(s).unwrap();
    *val_ref = *_cnt;
    *_cnt += 1;
    return mp_valve[s];
}

fn dfs(u: usize,step: ll, vis: &Vec<bool>, dist: &Vec<Vec<ll>>, fr: &Vec<ll>) -> ll{
    let n = dist.len();
    let mut ma = 0;
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
        let cur = dfs(v, step+ (*dist)[u][v], &n_vis, dist, fr);
        ma = std::cmp::max(ma, cur);
    }
    return ma;
}

fn main() {
    let mut mp_valve : HashMap<String, usize> = HashMap::new();
    let mut _cnt = 0;
    let mut G:Vec<Vec<usize>> = vec![Vec::new();100];
    let mut fr: Vec<ll> = vec![0;100];
    'input_loop: loop {
        let mut s = String::new();
        if rdln(&mut s) {
            break 'input_loop;
        }
        let tokens : Vec<String> = s.split(';').map(String::from).collect();
        let valve_pos = s.find("Valve ").unwrap() + 6; 
        let valve : String = s[valve_pos..valve_pos + 1].to_string();
        let flow_rate_pos = tokens[0].find("rate=").unwrap() + 5;
        let flow_rate = tokens[0][flow_rate_pos..].to_string();
        let to_valves_pos = s.find("valves ").unwrap() + 7;
        let to_valves = s[to_valves_pos..].to_string();
        let u = get_valve(&valve, &mut mp_valve, &mut _cnt);
        for v_s in to_valves.split(", ") {
            let v = get_valve(&v_s.to_string(), &mut mp_valve, &mut _cnt);
            G[u].push(v);
        }
        let fr_ref = fr.get_mut(u).unwrap();
        *fr_ref = flow_rate.parse().unwrap();
    }
    let n = _cnt;
    let mut dist: Vec<Vec<ll>> = vec![vec![0; n]; n];
    //flyod warshall
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
}
