use std::{collections::HashMap, env, fs::File, io::Read};

fn main() {
    println!("{}", get_total_steps());
}

fn get_total_steps() -> usize {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut directions: Vec<char> = vec![];
    let mut route_map = HashMap::new();

    for (index, line) in text.lines().enumerate() {
        if index == 0 {
            directions.append(&mut line.trim().chars().collect::<Vec<_>>());
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let (key, route_str) = line.split_once(" = ").unwrap();
        let route = route_str[1..route_str.len() - 1].split_once(", ").unwrap();
        route_map.insert(key, route);
    }

    let mut position = "AAA";
    let mut step = 0_usize;
    while position != "ZZZ" {
        let curr_route = route_map.get(position).unwrap();
        let direction_way = directions[step % directions.len()];
        position = match direction_way {
            'L' => curr_route.0,
            _ => curr_route.1,
        };
        step += 1;
    }

    step
}
