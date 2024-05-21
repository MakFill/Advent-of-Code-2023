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
    for mut history in history_vec {
        let mut history_val = *history.last().unwrap_or(&0);
        while history.iter().any(|i| i != &0) {
            let mut curr_history = vec![];
            for window in history.windows(2) {
                curr_history.push(window[1] - window[0]);
            }
            history_val += curr_history.last().unwrap_or(&0);
            history.clear();
            history.append(&mut curr_history);
        }
        res += history_val;
    }

    res
}
