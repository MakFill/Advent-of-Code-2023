use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_values_sum());
}

fn get_values_sum() -> i32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut history_vec = vec![];

    text.lines().for_each(|line| {
        let history = line
            .trim()
            .split_ascii_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        history_vec.push(history);
    });

    let mut res = 0;
    for history in history_vec {
        let first_val = *history.first().unwrap_or(&0);
        res += get_first_val_rec(history, first_val);
    }

    res
}

fn get_first_val_rec(data: Vec<i32>, val: i32) -> i32 {
    if data.iter().all(|i| *i == 0) {
        val
    } else {
        let mut curr_data = vec![];
        for window in data.windows(2) {
            curr_data.push(window[1] - window[0]);
        }
        let first = *curr_data.first().unwrap_or(&0);
        val - get_first_val_rec(curr_data, first)
    }
}
