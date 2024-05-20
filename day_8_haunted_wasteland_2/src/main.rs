use std::{collections::HashMap, env, fs::File, io::Read};

fn main() {
    println!("{}", get_total_steps());
}

fn get_total_steps() -> u64 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut directions: Vec<char> = vec![];
    let mut route_map = HashMap::new();
    let mut start_positions = vec![];

    for (index, line) in text.lines().enumerate() {
        if index == 0 {
            directions.append(&mut line.trim().chars().collect::<Vec<_>>());
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let (key, route_str) = line.split_once(" = ").unwrap();
        if key.ends_with('A') {
            start_positions.push((key, 0, false));
        }
        let route = route_str[1..route_str.len() - 1].split_once(", ").unwrap();
        route_map.insert(key, route);
    }

    let mut full_cycles_amount = 0;
    let mut step = 0;
    loop {
        let direction_way = directions[step % directions.len()];
        for (index, (position, cycle_length, is_full_cycle)) in
            start_positions.clone().iter_mut().enumerate()
        {
            if *is_full_cycle {
                continue;
            }
            let curr_route = route_map.get(position).unwrap();
            let pos = match direction_way {
                'L' => curr_route.0,
                _ => curr_route.1,
            };
            if pos.ends_with('Z') {
                *is_full_cycle = true;
                full_cycles_amount += 1;
            }
            *cycle_length += 1;
            start_positions[index] = (pos, *cycle_length, *is_full_cycle);
        }
        step += 1;
        if full_cycles_amount == start_positions.len() {
            break;
        }
    }

    let cycles_lengths = start_positions
        .iter()
        .map(|(_, length, _)| *length as u64)
        .collect::<Vec<_>>();

    cycles_lengths
        .into_iter()
        .reduce(|acc, curr| acc * curr / get_common_divisor(acc, curr))
        .unwrap()
}

fn get_common_divisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        get_common_divisor(b, a % b)
    }
}
