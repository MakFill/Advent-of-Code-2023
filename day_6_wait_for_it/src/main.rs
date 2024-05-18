use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_multiplyed_numbers());
}

fn get_multiplyed_numbers() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);

    let race_vec: Vec<Vec<u32>> = text
        .split('\n')
        .map(|line| {
            line.trim().split(':').collect::<Vec<_>>()[1]
                .split_ascii_whitespace()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut res = 1_u32;
    for (index, &time) in race_vec[0].iter().enumerate() {
        let distance = race_vec[1][index];
        let mut curr_res = 0_u32;
        for t in 1..time {
            if (time - t) * t > distance {
                curr_res += 1;
            }
        }
        res *= curr_res;
    }

    res
}
