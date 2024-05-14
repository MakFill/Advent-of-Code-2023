use std::{cmp::max, env, fs::File, io::Read};

fn main() {
    println!("{}", get_ids_sum());
}

fn get_ids_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("input.txt"));
    let mut text = String::new();
    let mut sum = 0;
    if let Ok(mut res) = file {
        let _ = res.read_to_string(&mut text);
        for s in text.split('\n') {
            let str_vec = s.split_ascii_whitespace().collect::<Vec<_>>();
            let mut max_red = 0_u32;
            let mut max_green = 0_u32;
            let mut max_blue = 0_u32;
            for (index, &item) in str_vec.iter().enumerate() {
                if item.starts_with("red") {
                    max_red = max(max_red, str_vec[index - 1].parse::<u32>().unwrap())
                }
                if item.starts_with("green") {
                    max_green = max(max_green, str_vec[index - 1].parse::<u32>().unwrap())
                }
                if item.starts_with("blue") {
                    max_blue = max(max_blue, str_vec[index - 1].parse::<u32>().unwrap())
                }
            }
            sum += max_red * max_green * max_blue;
        }
    }
    sum
}
