use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_sum())
}

fn get_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("src").join("input.txt"));
    let mut text = String::new();
    let mut sum = 0;
    if let Ok(mut res) = file {
        let _ = res.read_to_string(&mut text);
        text.split('\n').for_each(|s| {
            let mut first = 10;
            let mut last = 0;
            s.chars().filter_map(|c| c.to_digit(10)).for_each(|digit| {
                if first == 10 {
                    first = digit;
                }
                last = digit;
            });
            if first < 10 {
                sum += first * 10 + last;
            }
        });
    }
    sum
}
