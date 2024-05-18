use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_multiplyed_numbers());
}

fn get_multiplyed_numbers() -> usize {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut time = 0;
    let mut distance = 0;

    text.split('\n').enumerate().for_each(|(index, line)| {
        let res = line.trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .replace(' ', "")
            .parse::<u64>()
            .unwrap();
        if index == 0 {
            time = res;
        } else {
            distance = res;
        }
    });

    (1..time).filter(|t| (time - t) * t > distance).count()
}
